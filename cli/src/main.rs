use std::{env, path::PathBuf};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() {
    let core = flowagent_core::Core::new().await.unwrap();
    let core = core.initialize().await.unwrap();

    // Store core mutably so model/session changes can be persisted
    let core = core
        .create_session(env::current_dir().unwrap_or(PathBuf::from("/")))
        .await
        .unwrap();

    println!("Session started: {}", core.session_id());
    println!("Commands: /model, /models, /set-model <id number>, /exit\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        stdout.write_all(b"> ").await.unwrap();
        stdout.flush().await.unwrap();

        line.clear();
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        // --- Command Parsing ---
        match input {
            // Check active model
            "/model" => {
                println!("Current model: {}", core.current_model().name());
            }

            // List all available models
            "/models" => {
                println!("Available models:");
                for (index, model) in core.model_options().iter().enumerate() {
                    println!("{} - {}", index + 1, model.name());
                }
            }

            // Exit application
            "/exit" | "/quit" => {
                println!("Goodbye!");
                break;
            }

            // Parameterized commands
            _ if input.starts_with("/set-model ") => {
                let model_number = input.trim_start_matches("/set-model ").trim();
                let index = model_number.parse::<usize>().unwrap() - 1;

                todo!()
            }

            // Unknown slash commands
            _ if input.starts_with('/') => {
                println!("Unknown command: {}", input);
                println!("Available commands: /model, /models, /set-model <name>, /exit");
            }

            // General prompt handling
            _ => {
                println!("Sending to [{}]: {}", core.current_model().name(), input);
                // Core message dispatch logic here
            }
        }
    }
}
