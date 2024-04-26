use diesel::prelude::*;

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: String,
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub mapping: serde_json::Value,
    pub moderation_results: Vec<serde_json::Value>,
    pub current_node: String,
    pub plugin_ids: Option<Vec<String>>,
    pub conversation_id: String,
    pub conversation_template_id: Option<String>,
    pub gizmo_id: Option<String>,
    pub is_archived: bool,
    pub safe_urls: Vec<String>,
    pub default_model_slug: Option<String>
}

