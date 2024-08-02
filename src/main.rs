mod api;
mod config;
mod db;

use api::conversation;
use axum::{
    extract::{DefaultBodyLimit, Extension},
    routing::get,
    Router,
};
use config::Settings;
use db::postgres;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use ollama_rs::Ollama;
use std::sync::Arc;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct InferenceEngine {
    engine: Ollama,
    model: String,
}

#[derive(Clone, Debug)]
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    engine: InferenceEngine,
}

#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().expect("Loading Configuration failed");
    let db_pool = postgres::establish_connection(settings.database.url);
    let engine = InferenceEngine {
        engine: Ollama::new(settings.model.address, settings.model.port),
        model: settings.model.name,
    };

    let shared_state = Arc::new(AppState {
        db_pool: db_pool,
        engine: engine,
    });

    tracing_subscriber::registry()
        .with(settings.log_levels)
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();

    let app = Router::new()
        .route(
            "/",
            get(conversation::upload_form).post(conversation::upload_handler),
        )
        .layer(Extension(shared_state))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(settings.api.request_body_limit))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.api.address, settings.api.port))
            .await
            .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
