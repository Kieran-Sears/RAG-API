mod config;
mod api;
mod inference;

use std::sync::Arc;
use axum::{
    extract::{Extension, DefaultBodyLimit},
    routing::get,
    Router
};
use inference::engine::{InferenceEngine, create_inference_engine};

use api::conversation::{upload_form, upload_handler};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    inference_engine: Arc<dyn InferenceEngine + Send + Sync>
}

#[tokio::main]
async fn main() {

    let settings = config::load_config();
    let model_path = settings.get_string("model.path").unwrap();

    let inference_engine = create_inference_engine(model_path, "llm".to_string()).await;

    let shared_state = Arc::new(AppState {
        inference_engine,
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