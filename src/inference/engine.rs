use super::noop::NoOpInferenceEngine;
use crate::inference::models::*;
use std::{future::Future, pin::Pin};

pub trait InferenceEngine<VectorEncoding>: Send + Sync + Clone + Sized {
    fn new(model_path: String) -> Self
    where
        Self: Sized;
    fn infer(
        &self,
        prompt: String,
    ) -> Pin<Box<(dyn Future<Output = Result<InferResp, EngineError>> + Send + 'static)>>;
    fn encode(
        &self,
        document: String,
    ) -> Pin<Box<dyn Future<Output = Result<VectorEncodings, EngineError>> + Send + 'static>>;
}

impl InferenceEngine<VectorEncodings> for InferenceEngines {
    fn new(model_path: String) -> Self {
        InferenceEngines::NoOp(NoOpInferenceEngine::new(model_path))
    }

    fn infer(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = Result<InferResp, EngineError>> + Send + 'static>> {
        match self {
            InferenceEngines::Llm(engine) => engine.infer(prompt),
            InferenceEngines::NoOp(engine) => engine.infer(prompt),
            InferenceEngines::Ollama(engine) => engine.infer(prompt),
        }
    }

    fn encode(
        &self,
        document: String,
    ) -> Pin<Box<dyn Future<Output = Result<VectorEncodings, EngineError>> + Send + 'static>> {
        match self {
            InferenceEngines::Llm(engine) => engine.encode(document),
            InferenceEngines::NoOp(engine) => engine.encode(document),
            InferenceEngines::Ollama(engine) => engine.encode(document),
        }
    }
}
