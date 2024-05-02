use crate::inference::engine::{InferResp, InferErr, FloatVectors};
use crate::InferenceEngine;
use llm::{models::Llama, InferenceError, Model};
use llm_base::InferenceStats;
use std::future::IntoFuture;
use std::{future::Future, sync::Arc};
use tokio::task::JoinHandle;

pub struct LlmInferenceEngine {
    model: Arc<Llama>,
}

impl LlmInferenceEngine {
    pub fn new(model_path: String) -> Self {
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
}

impl InferenceEngine for LlmInferenceEngine {
    fn infer(
        &self,
        prompt: String,
    ) -> Box<dyn Future<Output = Result<InferResp, InferErr>> + Send> {
        let llama = &*self.model.clone();
        let future = {//async move {
            let mut session = llama.start_session(Default::default());
            let res = session.infer::<std::convert::Infallible>(
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
                }
            );

            match res {
                Ok(inferenceStats) => Ok(InferResp{result: inferenceStats.to_string()}),
                Err(inferenceError) => Err(InferErr{message: inferenceError.to_string()}) 
            }
        };
    
        Box::new(tokio::spawn(future))
    }

    fn encode(&self, document: String) -> Box<dyn Future<Output = dyn FloatVectors>> {
        let x = &self
            .model
            .clone()
            .vocabulary()
            .tokenize(document.as_str(), false);
        match x {
            Ok(v) => FloatVectors::new(v),
            Err(_) => todo!(),
        }
    }
}
