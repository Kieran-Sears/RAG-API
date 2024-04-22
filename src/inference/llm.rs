use crate::InferenceEngine;
use llm::{models::Llama, InferenceError, Model};
use llm_base::InferenceStats;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

pub struct LlmInferenceEngine {
    model: Arc<Mutex<Llama>>
}

impl LlmInferenceEngine {
    pub fn new(model_path: String) -> Self {
        LlmInferenceEngine {
            model: Arc::new(Mutex::new(
                llm::load::<Llama>(
                    std::path::Path::new(&model_path),
                    Default::default(),
                    llm::load_progress_callback_stdout,
                )
                .unwrap_or_else(|err| panic!("Failed to load model: {err}")),
            ))
        }
    }
}

impl InferenceEngine for LlmInferenceEngine {

    fn infer(
        &self,
        prompt: String,
    ) -> Pin<Box<dyn Future<Output = Result<InferenceStats, InferenceError>> + Send + '_>> {
        Box::pin(async move {
            let llama = &*self.model.lock().await;
            let mut session = llama.start_session(Default::default());
            session
                .infer::<std::convert::Infallible>(
                    llama,
                    &mut rand::thread_rng(),
                    &llm::InferenceRequest {
                        prompt: &prompt,
                        ..Default::default()
                    },
                    &mut Default::default(),
                    |t| {
                        print!("{t}");
                        Ok(())
                    },
                )
        })
    }
}
