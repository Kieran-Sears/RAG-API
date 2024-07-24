use crate::inference::models::{EngineError, InferResp, VectorEncoding};
use crate::InferenceEngine;
use std::{pin::Pin, future::Future};

#[derive(Debug, Clone)]
pub struct NoOpInferenceEngine;

pub struct NoOpEncoding;

impl VectorEncoding for NoOpEncoding {}

impl InferenceEngine<NoOpEncoding> for NoOpInferenceEngine {

    fn new(_model_path: String) -> Self {
        NoOpInferenceEngine
    }

    fn infer(&self, _prompt: String) -> Pin<Box<dyn Future<Output = Result<InferResp, EngineError>> + Send>> {
        Box::pin(async move { Ok(InferResp { result: "NoOp inference".to_string() }) })
    }

    fn encode(&self, _document: String) -> NoOpEncoding {
        NoOpEncoding
    }
}