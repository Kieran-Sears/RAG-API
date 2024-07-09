use crate::db::models::*;
use crate::db::postgres::insert_conversation;
use crate::inference::engine::InferenceEngine;
use crate::inference::models::*;
use crate::AppState;
use axum::{
    extract::{Extension, Multipart},
    response::Html,
};
use futures::Future;
use futures::stream::{self, StreamExt};
use serde_json::from_slice;
use std::sync::Arc;
use uuid::Uuid;

pub async fn upload_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

pub async fn upload_handler(state: Extension<Arc<AppState>>, mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let conversation_history: Vec<Conversation> =
            from_slice(&data).expect("JSON deserialization failed");
        let mut conn = state.db_pool.get().expect("Could not pool a db connector");

        let stored_ids: Vec<Uuid> = conversation_history
            .iter()
            .flat_map(|conversation| insert_conversation(&mut conn, &conversation))
            .collect();

        let _conversations: Vec<_> = stream::iter(conversation_history)
            .filter(|conversation| {
                let stored_ids = stored_ids.clone();
                let conversation_id = conversation.id.clone();

                async move { stored_ids.contains(&conversation_id) }
            })
            .then(|conversation| {
                let inference_engine = state.engine.clone();
                let conversation = conversation.clone();
                async move { is_question_or_answer(inference_engine, conversation) }
            })
            .collect()
            .await;
    }
}

async fn is_question_or_answer(
    state: InferenceEngines,
    conversation: Conversation,
) -> Result<Vec<(Uuid, bool)>, EngineError> {
    let context = "is this a question or an answer to a question? Answer with nothing other than `Yes` or `No`.";

    let futures: Vec<_> = conversation.mapping.iter().map(|(id, mapping)| {
        let m = mapping.clone();
        let message = m.message
        .expect(format!("Error: mapping {} does not have a message", mapping.id.to_string()).as_str()).content.parts.join("");
        let prompt = format!("{} {}", context, message);
        let s = state.clone();
        async move {
            let inference = s.infer(prompt).await?;
            let is_q_or_a: bool = match inference.result.as_str() {
                "yes" => Ok(true),
                "no" => Ok(false),
                _ => Err(EngineError::new(format!("Couldn't complete inference for {}", id).to_string()))
            }?;
            Ok::<(Uuid, bool), EngineError>((mapping.id, is_q_or_a))
        }
    }).collect();

    let results: Result<Vec<(Uuid, bool)>, EngineError> = stream::iter(futures)
    .then(|fut| fut)
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .collect();

    results
}
