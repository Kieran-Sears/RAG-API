mod config;
mod api;

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    extract::{Extension, DefaultBodyLimit},
    routing::get,
    Router
};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use llm::{Model, load, load_progress_callback_stdout, models::Llama};
use api::conversation::{upload_form, upload};
use rand;


#[derive(Clone)]
struct AppState {
    llama: Arc<Mutex<Option<Llama>>>,
}

pub trait Inference {
    async fn infer(&self, prompt: String) -> String;
}

impl Inference for AppState {
    async fn infer(&self, prompt: String) -> String {
        let llama = self.llama.lock().await;
        let llama = llama.as_ref().expect("Model not loaded");
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
            },
        );
    
        match res {
            Ok(result) => format!("\n\nInference stats:\n{result}"),
            Err(err) => format!("\n{err}"),
        }
    }
}

async fn load_model(llama: Arc<Mutex<Option<Llama>>>) {
    let model_path = "path_to_your_model_folder";
    let model = load::<Llama>(
        std::path::Path::new(model_path),
        Default::default(),
        load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    let mut guard = llama.lock().await;
    *guard = Some(model);
}

#[tokio::main]
async fn main() {

    let llama = Arc::new(Mutex::new(None));

    tokio::spawn(load_model(Arc::clone(&llama)));

    let shared_state = AppState {
        llama: Arc::clone(&llama),
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_multipart_form=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(upload_form).post(upload))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}
