#[tokio::main]
async fn main() {
    flowagent_core::Core::run().await;
}
