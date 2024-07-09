use diesel::{joinable, table};
use diesel::allow_tables_to_appear_in_same_query;

// table! {
//     conversation(id) {
//         id -> Uuid,
//         title -> Text,
//         create_time -> Double,
//         update_time -> Double,
//         mapping_id -> Array<Uuid>,
//         current_node -> Text,
//     }
// }

// table! {
//     mapping {
//         id -> Uuid,
//         message_ids -> Nullable<Array<Uuid>>,
//         parent -> Nullable<Uuid>,
//         children -> Array<Uuid>,
//     }
// }

// table! {
//     message {
//         id -> Uuid,
//         author_id -> Uuid,
//         create_time -> Nullable<Double>,
//         update_time -> Nullable<Double>,
//         content_id -> Uuid,
//         status -> Text,
//         end_turn -> Nullable<Bool>,
//         weight -> Double,
//         metadata -> Jsonb,
//         recipient -> Text,
//     }
// }

// table! {
//     author {
//         id -> Uuid,
//         role -> Text,
//         name -> Nullable<Text>,
//         metadata -> Jsonb,
//     }
// }

// table! {
//     content {
//         id -> Uuid,
//         content_type -> Text,
//         parts -> Nullable<Array<Text>>,
//     }
// }

// joinable!(conversation -> mapping (mapping_id));
// joinable!(mapping -> message (message_ids));
// joinable!(message -> author (author_id));
// joinable!(message -> content (content_id));
// allow_tables_to_appear_in_same_query!(conversation, mapping, message, author);


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
        default_model_slug -> Text,
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
        content_parts -> Nullable<Array<Text>>,
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