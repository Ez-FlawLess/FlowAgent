use std::{env, path::PathBuf};

use agent_client_protocol::{
    AcpAgent, Agent, Client, ConnectionTo, on_receive_notification,
    schema::{
        ProtocolVersion,
        v1::{
            ClientCapabilities, ContentBlock, FileSystemCapabilities, InitializeRequest,
            NewSessionRequest, PromptRequest, SessionConfigOptionValue, SessionNotification,
            SetSessionConfigOptionRequest, TextContent,
        },
    },
};
use thiserror::Error;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use crate::command::Command;

mod command;
mod connect_with;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

pub struct FlowAgentCore {
    client_handler: JoinHandle<Result<(), agent_client_protocol::Error>>,
    connection: ConnectionTo<Agent>,
}

impl FlowAgentCore {
    pub async fn new() -> Result<Self, NewFaCoreErr> {
        let opencode_cmd = if cfg!(target_os = "windows") {
            "opencode.cmd"
        } else {
            "opencode"
        };

        let agent = AcpAgent::from_args([opencode_cmd, "acp"]).map_err(NewFaCoreErr::Agent)?;

        let (cmd_sender, cmd_receiver) = mpsc::channel::<Command>(32);
        let (cx_sender, cx_receiver) = oneshot::channel();

        let handler = tokio::spawn(async move {
            Client
                .builder()
                .name("flowagent")
                .on_receive_notification(Self::on_receive_notification, on_receive_notification!())
                .connect_with(agent, async move |cx| {
                    let _ = cx_sender.send(cx.clone());
                    Self::connect_with(cx, cmd_receiver).await
                })
                .await
        });

        let Ok(cx) = cx_receiver.await else {
            let _ = cmd_sender.send(Command::Close).await;

            return match handler.await {
                Ok(Err(err)) => Err(NewFaCoreErr::Client(err)),
                Ok(Ok(())) | Err(_) => Err(NewFaCoreErr::Internal),
            };
        };

        return Ok(Self {
            client_handler: handler,
            connection: cx,
        });
    }

    async fn on_receive_notification(
        notification: SessionNotification,
        _cx: ConnectionTo<Agent>,
    ) -> Result<(), agent_client_protocol::Error> {
        println!("notification: {:?}", notification);
        Ok(())
    }

    async fn old_connect_with(cx: ConnectionTo<Agent>) -> Result<(), agent_client_protocol::Error> {
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

#[derive(Debug, Error)]
pub enum NewFaCoreErr {
    #[error("failed to startup acp agent of opencode: {0}")]
    Agent(agent_client_protocol::Error),
    #[error("failed to create client for connecting to agent: {0}")]
    Client(agent_client_protocol::Error),
    #[error("there was an internal error")]
    Internal,
}

impl Drop for FlowAgentCore {
    fn drop(&mut self) {}
}
