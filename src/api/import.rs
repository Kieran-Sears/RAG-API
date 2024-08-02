use crate::db::postgres::insert_conversation;
use crate::AppState;
use crate::{db::models::*, InferenceEngine};
use anyhow::{Error, Result};
use axum::{
    extract::{Extension, Multipart},
    response::Html,
};
use futures::stream::{self, StreamExt};
use ollama_rs::generation::completion::request::GenerationRequest;
use serde_json::{from_slice, from_value, Value};
use std::sync::Arc;
use tracing::{debug, error, info, trace};
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

        trace!("conversation_history {:?}", conversation_history);

        let questions_and_answers: Vec<Result<Vec<(Uuid, ContentType)>, Error>> = stream::iter(conversation_history)
            .then(|conversation| {
                let s = state.clone();
                let e = state.engine.clone();
                async move {
                    let mut conn = s.db_pool.get().expect("Could not pool a db connector");
                    insert_conversation(&mut conn, &conversation)?;
                    Ok(is_question_or_answer(e, conversation).await?)
                }
            })
            .collect()
            .await;

        info!("\n\nQuestions And Answers:\n\n {:?}", questions_and_answers);
    }
}

async fn is_question_or_answer(
    state: InferenceEngine,
    conversation: Conversation,
) -> Result<Vec<(Uuid, ContentType)>, Error> {
    let context = "Answer with nothing other than `question` or `answer` or `neither` without punctuation, \
        is this a question, an answer to a question or neither a question or answer:\n";

    let futures = conversation.mapping.into_iter()
        .filter_map(|(id, mapping)| {
            mapping.message.map(|msg| (id, get_content_text(&msg.content.parts).join("")))
        })
        .map(|(id, message)| {
            let prompt = format!("{} {}", context, message);
            let inf_eng = state.clone();
            async move {
                let request = GenerationRequest::new(inf_eng.model, prompt);
                let inference = inf_eng.engine.generate(request).await?;
                let response = inference.response;
                let is_q_or_a = match response.as_str() {
                    "question" => ContentType::Question,
                    "answer" => ContentType::Answer,
                    "neither" => ContentType::Other,
                    unknown => {
                        return Err(Error::msg(format!(
                            "Couldn't complete inference for Mapping: {}, LLM answered with something other than `question`, `answer`, or `neither`:\n{}",
                            id, unknown
                        )))
                    }
                };
                Ok((id, is_q_or_a))
            }
        });

    let results: Vec<Result<(Uuid, ContentType), Error>> = stream::iter(futures)
        .then(|fut| fut)
        .collect()
        .await;

    results.into_iter().collect()
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
