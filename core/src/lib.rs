use std::{env, path::PathBuf};

use agent_client_protocol::{
    AcpAgent, Agent, Client, ConnectionTo, on_receive_notification,
    schema::{
        ProtocolVersion,
        v1::{
            ClientCapabilities, ConfigOptionUpdate, ContentBlock, FileSystemCapabilities,
            InitializeRequest, NewSessionRequest, PromptRequest, SessionConfigOptionValue,
            SessionNotification, SessionUpdate, SetSessionConfigOptionRequest, TextContent,
        },
    },
};
use tokio::sync::oneshot;

pub struct FlowAgentCore {
    client: Client,
}

impl FlowAgentCore {
    pub async fn run() {
        let agent = AcpAgent::from_args(["opencode", "acp"]).unwrap();

        Client
            .builder()
            .name("flowagent")
            .on_receive_notification(Self::on_receive_notification, on_receive_notification!())
            .connect_with(agent, Self::connect_with)
            .await
            .unwrap();
    }

    async fn on_receive_notification(
        notification: SessionNotification,
        _cx: ConnectionTo<Agent>,
    ) -> Result<(), agent_client_protocol::Error> {
        println!("notification: {:?}", notification);
        Ok(())
    }

    async fn connect_with(cx: ConnectionTo<Agent>) -> Result<(), agent_client_protocol::Error> {
        let init_res = cx
            .send_request(
                InitializeRequest::new(ProtocolVersion::V1).client_capabilities(
                    ClientCapabilities::new().fs(FileSystemCapabilities::new()
                        .read_text_file(false)
                        .write_text_file(false)),
                ),
            )
            .block_task()
            .await
            .unwrap();

        println!("init done, agent info: {:?}", init_res.agent_info);

        let session_res = cx
            .send_request(NewSessionRequest::new(
                env::current_dir().unwrap_or(PathBuf::from("/")),
            ))
            .block_task()
            .await
            .unwrap();

        if let Some(config_options) = session_res.config_options.as_deref() {
            for config_option in config_options {
                if config_option.id.0.as_ref() == "model" {
                    // println!("list of models: {:#?}", config_option.kind);
                }
            }
        }

        let session_id = session_res.session_id;

        println!("created session with id {}", session_id);

        let config_model_res = cx
            .send_request(SetSessionConfigOptionRequest::new(
                session_id.clone(),
                "model",
                SessionConfigOptionValue::value_id("avalai/deepseek-v4-flash"),
            ))
            .block_task()
            .await
            .unwrap();

        println!("config_model_res: {:?}", config_model_res);

        let prompt_res = cx
            .send_request(PromptRequest::new(
                session_id,
                vec![ContentBlock::Text(TextContent::new("Hi"))],
            ))
            .block_task()
            .await
            .unwrap();

        println!("prompt done with reason: {:?}", prompt_res.stop_reason);

        Ok(())
    }
}
