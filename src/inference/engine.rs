
use std::{future::Future, pin::Pin, sync::Arc};

use llm::InferenceError;
use llm_base::InferenceStats;
use crate::inference::llm::LlmInferenceEngine;

pub trait InferenceEngine {
    fn infer(&self, prompt: String) ->  Pin<Box<dyn Future<Output = Result<InferenceStats, InferenceError>> + Send + '_>>;
}

pub async fn create_inference_engine(model_path: String, framework: String) -> Arc<dyn InferenceEngine + Send + Sync> {
    match framework.as_str() {
        "llm" => {
            let engine = LlmInferenceEngine::new(model_path);
            let arc = Arc::new(engine);
            return arc as Arc<dyn InferenceEngine + Send + Sync>;
        }
        _ => panic!("Invalid inference engine specified"),
    }
}