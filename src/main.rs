use std::io::Write;
use llm::{Model, load, load_progress_callback_stdout, models::Llama};
use rand;

mod config;

fn main() {

    // load config for model path etc.

    match config::load_config() {
        Ok(app_config) => {
            let model_table =  app_config.get_table("model").unwrap();
            

            for (key, value) in &model_table {
                println!("Key: {}, Value: {}", key, value);
            }

            let folder_path: String = model_table.get("path").unwrap().to_string();
            println!("Folder path: {}", folder_path);

            // load a GGML model from disk
            let llama = load::<Llama>(
                // path to GGML file
                std::path::Path::new(&folder_path),
                // llm::ModelParameters
                Default::default(),
                // load progress callback
                load_progress_callback_stdout,
            )
            .unwrap_or_else(|err| panic!("Failed to load model: {err}"));

            // use the model to generate text from a prompt
            let mut session = llama.start_session(Default::default());
            let res = session.infer::<std::convert::Infallible>(
                // model to use for text generation
                &llama,
                // randomness provider
                &mut rand::thread_rng(),
                // the prompt to use for text generation, as well as other
                // inference parameters
                &llm::InferenceRequest {
                    prompt: "Rust is a cool programming language because",
                    ..Default::default()
                },
                // llm::OutputRequest
                &mut Default::default(),
                // output callback
                |t| {
                    print!("{t}");
                    std::io::stdout().flush().unwrap();

                    Ok(())
                },
            );

            match res {
                Ok(result) => println!("\n\nInference stats:\n{result}"),
                Err(err) => println!("\n{err}"),
            }

        }
        Err(err) => eprintln!("Error loading configuration: {}", err),
    }

}
