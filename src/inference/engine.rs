
use std::{future::Future, pin::Pin, sync::Arc};

use llm::InferenceError; // <- shouldn't be here
use llm_base::InferenceStats; // <- shouldn't be here
use crate::inference::llm::LlmInferenceEngine;

pub trait InferenceEngine {
    // todo: change return type to be more generic and handle conversions within implementation definitions
    fn infer(&self, prompt: String) ->  Pin<Box<dyn Future<Output = Result<InferenceStats, InferenceError>> + Send + '_>>;
}

pub async fn create_inference_engine(model_path: String, framework: String) -> Arc<dyn InferenceEngine + Send + Sync> {
    match framework.as_str() {
        "llm" => return Arc::new(LlmInferenceEngine::new(model_path)) as Arc<dyn InferenceEngine + Send + Sync>,
        _ => panic!("Invalid inference engine specified"),
    }
}