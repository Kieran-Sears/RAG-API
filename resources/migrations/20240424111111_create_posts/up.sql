CREATE TABLE items (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    create_time DOUBLE PRECISION NOT NULL,
    update_time DOUBLE PRECISION NOT NULL,
    mapping JSONB NOT NULL,
    moderation_results JSONB[] NOT NULL,
    current_node TEXT NOT NULL,
    plugin_ids TEXT[],
    conversation_id TEXT NOT NULL,
    conversation_template_id TEXT,
    gizmo_id TEXT,
    is_archived BOOLEAN NOT NULL,
    safe_urls TEXT[] NOT NULL,
    default_model_slug TEXT
);