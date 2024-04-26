diesel::table! {
    conversations(id) {
        id -> Text,
        title -> Text,
        create_time -> Double,
        update_time -> Double,
        mapping -> Jsonb,
        moderation_results -> Array<Jsonb>,
        current_node -> Text,
        plugin_ids -> Nullable<Array<Text>>,
        conversation_id -> Text,
        conversation_template_id -> Nullable<Text>,
        gizmo_id -> Nullable<Text>,
        is_archived -> Bool,
        safe_urls -> Array<Text>,
        default_model_slug -> Nullable<Text>
    }
}