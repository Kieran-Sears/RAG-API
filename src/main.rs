use std::sync::Arc;
use tokio::sync::Mutex;
use rocket::{get, post, routes, Rocket};
use rocket::State;
use rocket_contrib::json::Json;
use llm::{Model, load, load_progress_callback_stdout, models::Llama};
use rand;

mod config;

#[derive(Clone)]
struct AppState {
    llama: Arc<Mutex<Option<Llama>>>,
}

#[get("/health")]
fn health_check() -> &'static str {
    "Service is up and running!"
}

#[post("/infer", data = "<prompt>")]
async fn infer(state: &State<AppState>, prompt: Json<String>) -> String {
    let llama = state.llama.lock().await;
    let llama = llama.as_ref().expect("Model not loaded");
    let mut session = llama.start_session(Default::default());
    let res = session.infer::<std::convert::Infallible>(
        llama,
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: &prompt,
            ..Default::default()
        },
        &mut Default::default(),
        |t| {
            print!("{t}");
            std::io::stdout().flush().unwrap();
            Ok(())
        },
    );

    match res {
        Ok(result) => format!("\n\nInference stats:\n{result}"),
        Err(err) => format!("\n{err}"),
    }
}

fn rocket() -> Rocket {
    let llama = Arc::new(Mutex::new(None));

    tokio::spawn(async move {
        let model_path = "path_to_your_model_folder";
        let model = load::<Llama>(
            std::path::Path::new(model_path),
            Default::default(),
            load_progress_callback_stdout,
        )
        .unwrap_or_else(|err| panic!("Failed to load model: {err}"));
        let mut guard = llama.lock().await;
        *guard = Some(model);
    });

    let state = AppState {
        llama: Arc::clone(&llama),
    };

    rocket::ignite()
    .manage(state)
    .mount("/", routes![health_check, infer, upload])
}

#[tokio::main]
async fn main() {
    rocket().launch().await.unwrap();
}
