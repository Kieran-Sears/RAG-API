use crate::db::models::*;
use crate::db::postgres::insert_conversation;
use crate::inference::engine::InferenceEngine;
use crate::inference::models::*;
use crate::AppState;
use axum::{
    extract::{Extension, Multipart},
    response::Html,
};
use futures::stream::{self, StreamExt};
use serde_json::{from_slice, from_value, Value};
use std::sync::Arc;
use tracing::{trace, debug, error, info};
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
        debug!(target: "api::conversation", "Entered upload_handler");
        let data = field.bytes().await.unwrap();
        let conversation_history: Vec<Conversation> =
            from_slice(&data).expect("JSON deserialization failed");
        let mut conn = state.db_pool.get().expect("Could not pool a db connector");

        trace!("conversation_history {:?}", conversation_history);

        let stored_ids: Vec<Uuid> = conversation_history
            .iter()
            .flat_map(|conversation| insert_conversation(&mut conn, &conversation))
            .collect();

        debug!("successfully stored conversations: {:?}", stored_ids);

        let questions_and_answers: Vec<_> = stream::iter(conversation_history)
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

        let results: Vec<Result<Vec<(uuid::Uuid, bool)>, EngineError>> =
            futures::future::join_all(questions_and_answers).await;
        let x: Vec<Result<Vec<(uuid::Uuid, bool)>, EngineError>> =
            results.into_iter().collect::<Vec<_>>();
        info!("\n\nQuestions And Answers:\n\n {:?}", x);
    }
}

async fn is_question_or_answer(
    state: InferenceEngines,
    conversation: Conversation,
) -> Result<Vec<(Uuid, bool)>, EngineError> {
    let context = "Answer with nothing other than `Yes` or `No`, is this a question or an answer to a question:\n";

    let futures: Vec<_> = conversation
        .mapping
        .into_iter()
        .filter_map(|(id, mapping)| mapping.message.map(|msg| (id, msg))).collect::<Vec<(Uuid, Message)>>()
        .into_iter()
        .map(|(mapping_id, message)| {
            let message = get_content_text(&message.clone().content.parts)
            .join("");
            let prompt = format!("{} {}", context, message);
            let s = state.clone();
            async move {
                let p = prompt.clone();
                let inference = s.infer(prompt).await?;
                let is_q_or_a: bool = match inference.result.as_str() {
                    "yes" => {
                        debug!("Prompt given to LLM for (question/answer) evaluation:\n{}\n\n Inference answer to the prompt:\n{}\n\n", p.to_string(), inference.result.to_string());
                        Ok(true)
                    }
                    "no" => Ok(false),
                    _ => Err(EngineError::new(
                        format!("Couldn't complete inference for Mapping: {}", mapping_id).to_string(),
                    )),
                }?;
                Ok::<(Uuid, bool), EngineError>((mapping_id, is_q_or_a))
            }
        })
        .collect();

    let results: Result<Vec<(Uuid, bool)>, EngineError> = stream::iter(futures)
        .then(|fut| fut)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect();

    results
}

fn get_content_text(items: &Option<Vec<Value>>) -> Vec<String> {
    items
        .clone()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|part| match from_value::<ContentPart>(part) {
            Ok(ContentPart::Text(text)) => Some(text),
            Ok(ContentPart::Object(_)) => None,
            Err(e) => {
                error!("Failed to deserialize content part: {}", e);
                None
            }
        })
        .collect()
}
