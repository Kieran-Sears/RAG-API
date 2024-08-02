use crate::db::schema::{conversations, mappings, messages};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Question,
    Answer,
    Other,
}

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = conversations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbConversation {
    id: Uuid,
    title: String,
    create_time: f64,
    update_time: f64,
    moderation_results: Option<serde_json::Value>,
    current_node: Option<Uuid>,
    plugin_ids: Option<Vec<Uuid>>,
    conversation_id: Option<Uuid>,
    conversation_template_id: Option<Uuid>,
    gizmo_id: Option<Uuid>,
    is_archived: bool,
    safe_urls: Option<Vec<String>>,
    default_model_slug: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub create_time: f64,
    pub update_time: f64,
    pub moderation_results: Option<Value>,
    pub current_node: Option<Uuid>,
    pub plugin_ids: Option<Vec<Uuid>>,
    pub conversation_id: Option<Uuid>,
    pub conversation_template_id: Option<Uuid>,
    pub gizmo_id: Option<Uuid>,
    pub is_archived: bool,
    pub safe_urls: Option<Vec<String>>,
    pub default_model_slug: Option<String>,
    pub mapping: std::collections::HashMap<Uuid, Mapping>,
}

impl From<Conversation> for DbConversation {
    fn from(conversation: Conversation) -> Self {
        DbConversation {
            id: conversation.id,
            title: conversation.title,
            create_time: conversation.create_time,
            update_time: conversation.update_time,
            moderation_results: conversation.moderation_results,
            current_node: conversation.current_node,
            plugin_ids: conversation.plugin_ids,
            conversation_id: conversation.conversation_id,
            conversation_template_id: conversation.conversation_template_id,
            gizmo_id: conversation.gizmo_id,
            is_archived: conversation.is_archived,
            safe_urls: conversation.safe_urls,
            default_model_slug: conversation.default_model_slug,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Mapping {
    pub id: Uuid,
    pub message: Option<Message>,
    pub parent: Option<Uuid>,
    pub children: Option<Vec<Uuid>>,
    pub content_type: Option<ContentType>,
}

#[derive(Insertable, Queryable, Debug, Identifiable)]
#[diesel(table_name = mappings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbMapping {
    id: Uuid,
    message: Option<Uuid>,
    parent: Option<Uuid>,
    children: Option<Vec<Uuid>>,
}

impl From<Mapping> for DbMapping {
    fn from(mapping: Mapping) -> Self {
        DbMapping {
            id: mapping.id,
            message: mapping.message.map(|msg| msg.id),
            parent: mapping.parent,
            children: mapping.children,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub id: Uuid,
    pub author: Author,
    pub create_time: Option<f64>,
    pub update_time: Option<f64>,
    pub content: Content,
    pub status: String,
    pub end_turn: Option<bool>,
    pub weight: f64,
    pub metadata: serde_json::Value,
    pub recipient: String,
}

#[derive(Insertable, Queryable, Identifiable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbMessage {
    pub id: Uuid,
    pub author_role: String,
    pub author_name: Option<String>,
    pub author_metadata: Value,
    pub create_time: Option<f64>,
    pub update_time: Option<f64>,
    pub content_type: String,
    pub content_parts: Option<Vec<Value>>,
    pub status: String,
    pub end_turn: Option<bool>,
    pub weight: f64,
    pub metadata: Value,
    pub recipient: String,
}

impl From<Message> for DbMessage {
    fn from(message: Message) -> Self {
        DbMessage {
            id: message.id,
            author_role: message.author.role,
            author_name: message.author.name,
            author_metadata: message.author.metadata,
            create_time: message.create_time,
            update_time: message.update_time,
            content_type: message.content.content_type,
            content_parts: message.content.parts,
            status: message.status,
            end_turn: message.end_turn,
            weight: message.weight,
            metadata: message.metadata,
            recipient: message.recipient,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Author {
    pub role: String,
    pub name: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Content {
    pub content_type: String,
    pub parts: Option<Vec<Value>>,
}



#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ContentPart {
    Text(String),
    Object(ContentPartObject),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ContentPartObject {
    content_type: String,
    asset_pointer: String,
    size_bytes: u64,
    width: u32,
    height: u32,
    fovea: Option<Value>,
    metadata: Option<Value>,
}