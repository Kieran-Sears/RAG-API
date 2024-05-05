use crate::inference::engine::{InferErr, InferResp, VectorEncoding};
use crate::InferenceEngine;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct NoOpInferenceEngine;

pub struct NoOpEncoding;

impl VectorEncoding for NoOpEncoding {}

impl InferenceEngine<NoOpEncoding> for NoOpInferenceEngine {

    fn new(_model_path: String) -> Self {
        NoOpInferenceEngine
    }

    fn infer(&self, _prompt: String) -> Box<dyn Future<Output = Result<InferResp, InferErr>> + Send> {
        Box::new(async move { Ok(InferResp { result: "NoOp inference".to_string() }) })
    }

    fn encode(&self, _document: String) -> NoOpEncoding {
        NoOpEncoding
    }
}