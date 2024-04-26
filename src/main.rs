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
use inference::engine::{InferenceEngine, create_inference_engine};

use api::conversation::{upload_form, upload_handler};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    inference_engine: Arc<dyn InferenceEngine + Send + Sync>
}

#[tokio::main]
async fn main() {

    let settings = config::load_config();

    let database_url = settings.get_string("database.url").unwrap();
    let db_pool = db::postgres::establish_connection(&database_url);

    //<test code>
    // use serde_json::json;
    
    // use crate::db::models::Item;
    // let new_item = Item {
    //     id: "123456".to_string(),
    //     title: "Sample Item".to_string(),
    //     create_time: 1620025200.123,
    //     update_time: 1620025300.456,
    //     mapping: json!({
    //         "key1": "value1",
    //         "key2": 42,
    //         "key3": ["a", "b", "c"]
    //     }),
    //     moderation_results: vec![
    //         json!({"result": "approved"}),
    //         json!({"result": "rejected"}),
    //     ],
    //     current_node: "node123".to_string(),
    //     plugin_ids: Some(vec!["plugin1".to_string(), "plugin2".to_string()]),
    //     conversation_id: "conversation456".to_string(),
    //     conversation_template_id: Some("template789".to_string()),
    //     gizmo_id: Some("gizmo987".to_string()),
    //     is_archived: false,
    //     safe_urls: vec![
    //         "https://example.com".to_string(),
    //         "https://example.org".to_string(),
    //     ],
    //     default_model_slug: Some("default".to_string()),
    // };

    // let item_id = db::postgres::create_item(&mut db_pool.get().expect("Could not pool a db connector"), &new_item).id;
    // db::postgres::search_item(&mut db_pool.get().expect("Could not pool a db connector"), &item_id);

    // </test code>

    let model_path = settings.get_string("model.path").unwrap();

    let inference_engine = create_inference_engine(model_path, "llm".to_string()).await;

    let shared_state = Arc::new(AppState {
        db_pool,
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