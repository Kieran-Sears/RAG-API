mod config;
mod api;
mod db;
mod inference;

use std::sync::Arc;
use axum::{
    extract::{Extension, DefaultBodyLimit},
    routing::get,
    Router
};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use inference::engine::InferenceEngine;
use inference::models::InferenceEngines;
use inference::llm::LlmInferenceEngine;
use api::conversation;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    engine: InferenceEngines
}

#[tokio::main]
async fn main() {

    let settings = config::load_config();
    let database_url = settings.get_string("database.url").unwrap();
    let db_pool = db::postgres::establish_connection(&database_url);
    let model_path = settings.get_string("model.path").unwrap();
    let engine = LlmInferenceEngine::new(model_path);
    let shared_state = Arc::new(AppState {db_pool, engine: InferenceEngines::Llm(engine)});
    
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_multipart_form=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(conversation::upload_form).post(conversation::upload_handler))
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