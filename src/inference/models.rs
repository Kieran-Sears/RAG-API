use super::{
    llm::{LLMEncoding, LlmInferenceEngine},
    noop::{NoOpEncoding, NoOpInferenceEngine},
};
use thiserror::Error;

pub trait VectorEncoding: Sized {}

#[derive(Clone)]
pub enum InferenceEngines {
    Llm(LlmInferenceEngine),
    NoOp(NoOpInferenceEngine),
}

pub enum VectorEncodings {
    Llm(LLMEncoding),
    NoOp(NoOpEncoding),
}

#[derive(Debug, Clone)]
pub struct InferResp {
    pub result: String,
}

#[derive(Debug, Error, Clone)]
pub enum EngineError {
    #[error("Engine Inference Error: {message}")]
    InferenceError { message: String },
}

impl EngineError {
    pub fn new(message: String) -> Self {
        EngineError::InferenceError { message }
    }
}
