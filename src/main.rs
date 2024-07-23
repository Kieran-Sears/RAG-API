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
use ::config::ext::ConfigurationBinder;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use inference::engine::InferenceEngine;
use inference::models::InferenceEngines;
use inference::llm::LlmInferenceEngine;
use db::postgres;
use api::conversation;
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{ EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};


#[derive(Clone, Debug)]
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    engine: InferenceEngines
}

#[tokio::main]
async fn main() {

    let settings = config::load_config();
    let database_url = settings.get("Database:Url").unwrap();
    let db_pool = postgres::establish_connection(database_url.to_string());
    let model_path = settings.get("model:path").unwrap();
    let engine = LlmInferenceEngine::new(model_path.to_string());
    let shared_state = Arc::new(AppState {db_pool, engine: InferenceEngines::Llm(engine)});
    let address = settings.get("Api:Address").unwrap();
    let port = settings.get("Api:Port").unwrap();
    let request_body_limit = settings.get_value::<usize>("Api:Request_Body_Limit").unwrap().unwrap();

    // todo replace with value from config
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let default = &format!("{}=INFO", env!("CARGO_PKG_NAME").replace("-", "_"));
        let default_filter = &format!("{},db=ERROR,api::upload_handler=INFO,tower_http=INFO", default).to_string();
        println!("No RUST_LOG environment variable set. Using default: {}", default_filter);
        default_filter.into()
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();

    let app = Router::new()
        .route("/", get(conversation::upload_form).post(conversation::upload_handler))
        .layer(Extension(shared_state))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(request_body_limit))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address.to_string(), port.to_string()))
    .await
    .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

}

