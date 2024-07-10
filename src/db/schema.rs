use diesel::{joinable, table};
use diesel::allow_tables_to_appear_in_same_query;

table! {
    conversations (id) {
        id -> Uuid,
        title -> Text,
        create_time -> Double,
        update_time -> Double,
        moderation_results -> Nullable<Jsonb>,
        current_node -> Nullable<Uuid>,
        plugin_ids -> Nullable<Array<Uuid>>,
        conversation_id -> Nullable<Uuid>,
        conversation_template_id -> Nullable<Uuid>,
        gizmo_id -> Nullable<Uuid>,
        is_archived -> Bool,
        safe_urls -> Nullable<Array<Text>>,
        default_model_slug -> Nullable<Text>,
    }
}

table! {
    messages (id) {
        id -> Uuid,
        author_role -> Text,
        author_name -> Nullable<Text>,
        author_metadata -> Nullable<Jsonb>,
        create_time -> Nullable<Double>,
        update_time -> Nullable<Double>,
        content_type -> Text,
        content_parts -> Nullable<Array<Jsonb>>,
        status -> Text,
        end_turn -> Nullable<Bool>,
        weight -> Double,
        metadata -> Jsonb,
        recipient -> Text,
    }
}

table! {
    mappings (id) {
        id -> Uuid,
        message -> Nullable<Uuid>,
        parent -> Nullable<Uuid>,
        children -> Nullable<Array<Uuid>>,
    }
}


joinable!(mappings -> messages (message));

allow_tables_to_appear_in_same_query!(
    conversations,
    messages,
    mappings,
);