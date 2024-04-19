mod config;
mod api;

use rand;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    extract::{Extension, DefaultBodyLimit},
    routing::get,
    Router
};
use llm::{
    Model, 
    load, 
    load_progress_callback_stdout, 
    models::Llama, 
    InferenceError};
use llm_base::InferenceStats;
use api::conversation::{upload_form, upload_handler};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    llama: Arc<Mutex<Option<Llama>>>,
}

pub trait Inference {
    fn infer(&self, prompt: String) -> impl std::future::Future<Output = Result<InferenceStats, InferenceError>> + Send;
}

impl Inference for AppState {
    async fn infer(&self, prompt: String) -> Result<InferenceStats, InferenceError> {
        let llama = self.llama.lock().await;
        let llama = llama.as_ref().expect("Model not loaded");
        let mut session = llama.start_session(Default::default());
        session.infer::<std::convert::Infallible>(
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
    }
}

async fn load_model(llama: Arc<Mutex<Option<Llama>>>, model_path: String) {
    let model = load::<Llama>(
        std::path::Path::new(&model_path),
        Default::default(),
        load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
    let mut guard = llama.lock().await;
    *guard = Some(model);
}

#[tokio::main]
async fn main() {

    let settings = config::load_config();
    let model_path = settings.get_string("model.path").unwrap();

    let llama = Arc::new(Mutex::new(None));

    tokio::spawn(load_model(Arc::clone(&llama), model_path));

    let shared_state = Arc::new(AppState {
        llama: Arc::clone(&llama),
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_multipart_form=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(upload_form).post(upload_handler))
        .layer(Extension(shared_state))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            250 * 1024 * 1024, /* 250mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}
