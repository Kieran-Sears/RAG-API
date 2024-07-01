extern crate serde_json;

use crate::db::{models::Item, postgres::create_item};
use crate::inference::engine::InferenceEngine;
use crate::AppState;
use axum::{
    extract::{Extension, Multipart},
    response::Html,
};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde_json::{from_slice, Value};
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;

pub async fn upload_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

pub async fn upload_handler(state: Extension<Arc<AppState>>, mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let items: Vec<Item> = from_slice(&data).unwrap();
        let mut conn = state.db_pool.get().expect("Could not pool a db connector");
        let messages = store_items(&items, &mut conn);

        for m in messages {
            let data: HashMap<String, Value> = serde_json::from_value(m.mapping.clone())
                .expect(&format!("Couldn't get mapping from message item {}", m.id));

            let root_id = data
            .iter()
            .find_map(|(id, obj)| {
                if obj["parent"].is_null() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .expect("No root object found with parent = null");

            let mut linked_list: LinkedList<String> = LinkedList::new();
            let mut current_id = root_id.clone();

            loop {
                linked_list.push_back(current_id.clone());
                let children = &data[&current_id]["children"];
                if children.is_array() && !children.as_array().unwrap().is_empty() {
                    current_id = children[0].as_str().unwrap().to_string();
                } else {
                    break;
                }
            }

            for id in linked_list {
                println!("Linked List ID: {}", id);
            }
        }
    }
}
// for item in messages {
//     let context = "is this a question or an answer to a question? Answer with nothing other than `Yes` or `No`.";
//     println!("{}", item.mapping);
//     let prompt = format!("{} {}", context, item.mapping.as_str().unwrap());
//     let response = state.engine.infer(prompt);

//     match response.await {
//         Ok(infer_resp) => {
//             // Handle the response
//             println!("Inference response: {:?}", infer_resp);
//         },
//         Err(infer_err) => {
//             // Handle the error
//             eprintln!("Inference error: {:?}", infer_err);
//         },
//     }
// }

fn store_items<'a>(
    items: &'a Vec<Item>,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> &'a Vec<Item> {
    // use serde_json::to_string_pretty;
    let mut _failed_items = Vec::new();
    let mut successful_items = Vec::new();

    for item in items {
        match create_item(conn, &item) {
            Ok(_) => {
                successful_items.push(item);
            }
            Err(err) => {
                // println!("{}", format!("Could not store item:\n{}\n\nError:\n{}", to_string_pretty(&item.id).unwrap(), err.to_string()));
                _failed_items.push((item, err));
            }
        }
    }
    items
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MessageContent {
    content_type: String,
    parts: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MessageNode {
    id: String,
    message: MessageContent,
    children: Option<Box<MessageNode>>,
}

impl Clone for MessageNode {
    fn clone(&self) -> Self {
        MessageNode {
            id: self.id.clone(),
            message: self.message.clone(),
            children: self.children.clone(),
        }
    }
}
