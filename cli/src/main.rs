use std::{env, path::PathBuf};

#[tokio::main]
async fn main() {
    let core = flowagent_core::Core::new().await.unwrap();

    let core = core.initialize().await.unwrap();
    println!("client name: {}", core.client_name());

    let core = core
        .create_session(env::current_dir().unwrap_or(PathBuf::from("/")))
        .await
        .unwrap();
    println!("session id: {}", core.session_id());
    println!("current model: {}", core.current_model().name());
}
