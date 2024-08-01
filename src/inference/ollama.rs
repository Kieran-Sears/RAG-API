use std::{future::Future, pin::Pin, sync::Arc};

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use super::{engine::InferenceEngine, models::{EngineError, InferResp, VectorEncoding, VectorEncodings}};
use tracing::{trace, error};


impl VectorEncoding for OllamaEncoding {}

pub struct OllamaEncoding {
    value: Vec<f64>,
}

impl OllamaEncoding {
    pub fn new(v: &Vec<f64>) -> Self {
        let value: Vec<f64> = v.iter().map(|&val|  val).collect();
        OllamaEncoding { value }
    }

    pub fn get(&self) -> Vec<f64> {
        self.value.iter().map(|val|  *val).collect()
    }
}

#[derive(Clone)]
pub struct OllamaInferenceEngine {
    model: Arc<Ollama>,
    model_name: &'static str
}

impl std::fmt::Debug for OllamaInferenceEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: Implement how ExternalLibraryStruct should be formatted
        write!(f, "ExternalLibraryStruct {{ id: {} }}", "someId")
    }
}

impl InferenceEngine<OllamaEncoding> for OllamaInferenceEngine {

    fn new(model_path: String) -> Self {
        OllamaInferenceEngine {
            model: Arc::new(Ollama::new("http://localhost".to_string(), 11434)),
            model_name: "llama3.1:latest"
        }
    }

    fn infer(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = Result<InferResp, EngineError>> + Send>> {
        let ollama = self.model.clone();
        let model = self.model_name.to_string();
        Box::pin(async move {
            let res = ollama.generate(GenerationRequest::new(model, prompt)).await;
            match res {
                Ok(inference) => {
                    trace!("inference_stats: {}", inference.response);
                    Ok(InferResp { result: inference.response.to_string() })
                },
                Err(inference_error) => {
                    error!("Inference Error: {}", inference_error);
                    Err(EngineError::InferenceError { message: inference_error.to_string() })
                }
            }
        })
    }

    fn encode(&self, document: String) -> Pin<Box<dyn Future<Output = Result<VectorEncodings, EngineError>> + Send>> {
       let m = self.model.clone();
       let n = self.model_name.to_string();
        Box::pin(async move {
        m.generate_embeddings(n, document, None).await
        .map(|val| VectorEncodings::Ollama(OllamaEncoding::new(&val.embeddings.to_vec())))
        .map_err(|err| EngineError::EncodingError { message: err.to_string() })
        })
    }
}
