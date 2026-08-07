use agent_client_protocol::{Agent, ConnectionTo};
use tokio::sync::mpsc;

use crate::{FlowAgentCore, command::Command};

impl FlowAgentCore {
    pub async fn connect_with(
        cx: ConnectionTo<Agent>,
        mut cmd_receiver: mpsc::Receiver<Command>,
    ) -> Result<(), agent_client_protocol::Error> {
        while let Some(cmd) = cmd_receiver.recv().await {
            match cmd {
                Command::Close => break,
                Command::GetModels => {}
            }
        }
        Ok(())
    }
}
