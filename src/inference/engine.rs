
use std::{future::Future, sync::Arc};
use crate::inference::llm::LlmInferenceEngine;

pub trait FloatVectors {
    fn precision(&self) -> &'static str;
}

impl FloatVectors for Vec<f32> {
    fn precision(&self) -> &'static str {
        "Single precision (f32)"
    }
}

impl FloatVectors for Vec<f64> {
    fn precision(&self) -> &'static str {
        "Double precision (f64)"
    }
}



#[derive(Debug, Clone)]
pub struct InferResp {
    pub result: String
}


#[derive(Debug, Clone)]
pub struct InferErr {
    pub message: String
}


pub trait InferenceEngine: Send + Sync {
    fn infer(&self, prompt: String) ->  Box<(dyn Future<Output = Result<InferResp, InferErr>> + Send + 'static)>;
    fn encode(&self, document: String) ->  Box<dyn Future<Output = dyn FloatVectors>>;
}

pub async fn create_inference_engine(model_path: String, framework: String) -> Arc<dyn InferenceEngine> {
    match framework.as_str() {
        "llm" => return Arc::new(LlmInferenceEngine::new(model_path)) as Arc<dyn InferenceEngine>,
        _ => panic!("Invalid inference engine specified"),
    }
}