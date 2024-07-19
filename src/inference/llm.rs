use crate::inference::models::*;
use crate::InferenceEngine;
use llm::{models::Llama, Model};
use std::{future::Future, pin::Pin, sync::Arc};
use tracing::{trace, error};

impl VectorEncoding for LLMEncoding {}

pub struct LLMEncoding {
    value: Vec<(Vec<u8>, i32)>,
}

impl LLMEncoding {
    pub fn new(v: &Vec<(&[u8], i32)>) -> Self {
        let value: Vec<(Vec<u8>, i32)> = v.iter().map(|&(bytes, val)| (bytes.to_vec(), val)).collect();
        LLMEncoding { value }
    }

    pub fn get(&self) -> Vec<(&[u8], i32)> {
        self.value.iter().map(|(bytes, val)| (bytes.as_slice(), *val)).collect()
    }
}

#[derive(Clone)]
pub struct LlmInferenceEngine {
    model: Arc<Llama>,
}

impl std::fmt::Debug for LlmInferenceEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: Implement how ExternalLibraryStruct should be formatted
        write!(f, "ExternalLibraryStruct {{ id: {} }}", "someId")
    }
}

impl InferenceEngine<LLMEncoding> for LlmInferenceEngine {

    fn new(model_path: String) -> Self {
        LlmInferenceEngine {
            model: Arc::new(
                llm::load::<Llama>(
                    std::path::Path::new(&model_path),
                    Default::default(),
                    llm::load_progress_callback_stdout,
                )
                .unwrap_or_else(|err| panic!("Failed to load model: {err}")),
            ),
        }
    }

    fn infer(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = Result<InferResp, EngineError>> + Send>> {
        let llama = self.model.clone();
        Box::pin(async move {
            let mut session = llama.start_session(Default::default());
            let res = session.infer::<std::convert::Infallible>(
                &*llama,
                &mut rand::thread_rng(),
                &llm::InferenceRequest {
                    prompt: &prompt,
                    ..Default::default()
                },
                &mut Default::default(),
                |t| {
                    trace!("{t}");
                    Ok(())
                },
            );
            match res {
                Ok(inference_stats) => {
                    trace!("inference_stats: {}", inference_stats);
                    Ok(InferResp { result: inference_stats.to_string() })
                },
                Err(inference_error) => {
                    error!("Inference Error: {}", inference_error);
                    Err(EngineError::InferenceError { message: inference_error.to_string() })
                }
            }
        })
    }

    fn encode(&self, document: String) -> LLMEncoding {
        let x = &self.model.vocabulary().tokenize(document.as_str(), false);
        match x {
            Ok(v) => LLMEncoding::new(&v),
            Err(_) => todo!(),
        }
    }
}
