use crate::AppState;
use std::sync::Arc;
use axum::{
    extract::{Extension, Multipart},
    response::Html
};

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
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );

        let hello_world = String::from("Hello, world!");
        let response = state.inference_engine.infer(hello_world).await;

        match response {
            Ok(result) => format!("\n\nInference stats:\n{result}"),
            Err(err) => format!("\n{err}"),
        };
    }
}