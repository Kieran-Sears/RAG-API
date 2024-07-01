
use std::{future::Future, pin::Pin};

use super::{llm::{LLMEncoding, LlmInferenceEngine}, noop::{NoOpEncoding, NoOpInferenceEngine}};

pub trait VectorEncoding: Sized {}

#[derive(Debug, Clone)]
pub struct InferResp {
    pub result: String
}

#[derive(Debug, Clone)]
pub struct InferErr {
    pub message: String
}

pub trait InferenceEngine<VectorEncoding>: Send + Sync + Clone + Sized {
    fn new(model_path: String) -> Self where Self: Sized;
    fn infer(&self, prompt: String) ->  Pin<Box<(dyn Future<Output = Result<InferResp, InferErr>> + Send + 'static)>>;
    fn encode(&self, document: String) -> VectorEncoding;
}


impl InferenceEngine<VectorEncodings> for InferenceEngines {

    fn new(model_path: String) -> Self {
        InferenceEngines::NoOp(NoOpInferenceEngine::new(model_path))
    }

    fn infer(&self, prompt: String) -> Pin<Box<dyn Future<Output = Result<InferResp, InferErr>> + Send + 'static>> {
        match self {
            InferenceEngines::Llm(engine) => engine.infer(prompt),
            InferenceEngines::NoOp(engine) => engine.infer(prompt),
        }
    }

    fn encode(&self, document: String) -> VectorEncodings {
        match self {
            InferenceEngines::Llm(engine) => VectorEncodings::Llm(engine.encode(document)),
            InferenceEngines::NoOp(engine) => VectorEncodings::NoOp(engine.encode(document)),
        }
    }
}

#[derive(Clone)]
pub enum InferenceEngines {
    Llm(LlmInferenceEngine),
    NoOp(NoOpInferenceEngine),
    // Add more variants for other implementations as needed
}

pub enum VectorEncodings {
    Llm(LLMEncoding),
    NoOp(NoOpEncoding),
}