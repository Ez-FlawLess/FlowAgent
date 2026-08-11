#[tokio::main]
async fn main() {
    let core = flowagent_core::Core::new().await.unwrap();
    let core = core.initialize().await.unwrap();
    println!("client name: {}", core.client_name());
}
