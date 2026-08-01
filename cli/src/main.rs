use flowagent_core::FlowAgentCore;

#[tokio::main]
async fn main() {
    FlowAgentCore::run().await;
}
