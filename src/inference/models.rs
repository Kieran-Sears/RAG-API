use super::{
    llm::{LLMEncoding, LlmInferenceEngine},
    noop::{NoOpEncoding, NoOpInferenceEngine},
    ollama::{OllamaEncoding, OllamaInferenceEngine}
};
use thiserror::Error;

pub trait VectorEncoding: Sized {}

#[derive(Clone, Debug)]
pub enum InferenceEngines {
    Llm(LlmInferenceEngine),
    NoOp(NoOpInferenceEngine),
    Ollama(OllamaInferenceEngine),
}

pub enum VectorEncodings {
    Llm(LLMEncoding),
    NoOp(NoOpEncoding),
    Ollama(OllamaEncoding),
}

#[derive(Debug, Clone)]
pub struct InferResp {
    pub result: String,
}

#[derive(Debug, Error, Clone)]
pub enum EngineError {
    #[error("Engine Inference Error: {message}")]
    InferenceError { message: String },
    #[error("Engine Encoding Error: {message}")]
    EncodingError { message: String },
}

// impl EngineError {
//     pub fn new(message: String) -> Self {
//         EngineError::InferenceError { message }
//     }

//     pub fn new (message: String) -> Self {
//         EngineError::EncodingError { message }
//     }
// }

