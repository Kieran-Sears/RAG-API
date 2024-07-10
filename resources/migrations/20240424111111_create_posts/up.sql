CREATE TABLE conversations (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    create_time DOUBLE PRECISION NOT NULL,
    update_time DOUBLE PRECISION NOT NULL,
    moderation_results JSONB,
    current_node UUID,
    plugin_ids UUID[],
    conversation_id UUID,
    conversation_template_id UUID,
    gizmo_id UUID,
    is_archived BOOLEAN NOT NULL,
    safe_urls TEXT[],
    default_model_slug TEXT NOT NULL
);

CREATE TABLE messages (
    id UUID PRIMARY KEY,
    author_role TEXT NOT NULL,
    author_name TEXT,
    author_metadata JSONB,
    create_time DOUBLE PRECISION,
    update_time DOUBLE PRECISION,
    content_type TEXT NOT NULL,
    content_parts TEXT[],
    status TEXT NOT NULL,
    end_turn BOOLEAN,
    weight DOUBLE PRECISION NOT NULL,
    metadata JSONB,
    recipient TEXT NOT NULL
);

CREATE TABLE mappings (
    id UUID PRIMARY KEY,
    message_id UUID REFERENCES messages(id),
    parent UUID,
    children UUID[]
);