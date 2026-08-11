use std::process::Stdio;

use getset::Getters;
use thiserror::Error;
use tokio::{
    io,
    process::{Child, ChildStdin, ChildStdout, Command},
};

use crate::{
    json_rpc::{JsonRpc, RpcSendErr},
    schemes::init::{ClientInfo, InitReq, InitRes},
    shared::acp_protocl_version::AcpProtocolVersion,
};

mod json_rpc;
mod schemes;
mod shared;
mod utils;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("this code only runs on Windows, Linux, and macOS");

#[derive(Getters)]
pub struct Core {
    #[getset(get = "pub")]
    client_name: String,
    acp_process: Child,
    rpc: JsonRpc<ChildStdin, ChildStdout>,
}

impl Core {
    pub async fn new() -> Result<Self, NewCoreErr> {
        let mut acp_process = Command::new("opencode")
            .arg("acp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(NewCoreErr::SpawnProc)?;

        let stdin = acp_process.stdin.take().ok_or(NewCoreErr::Internal)?;
        let stdout = acp_process.stdout.take().ok_or(NewCoreErr::Internal)?;

        let rpc = JsonRpc::new(stdin, stdout);

        let response = rpc
            .send::<_, InitRes>(InitReq {
                acp_protocol_version: AcpProtocolVersion::V1,
                client_info: ClientInfo {
                    name: "flowagent".to_string(),
                    title: Some("Flow Agent".to_string()),
                    version: "1.0.0".to_string(),
                },
            })
            .await
            .map_err(NewCoreErr::Init)?;

        if response.acp_protocol_version != AcpProtocolVersion::V1 {
            return Err(NewCoreErr::UnsupportedAcpVersion(
                response.acp_protocol_version,
            ));
        }

        Ok(Self {
            client_name: response
                .agent_info
                .title
                .unwrap_or(response.agent_info.name),
            acp_process,
            rpc,
        })
    }
}

#[derive(Debug, Error)]
pub enum NewCoreErr {
    #[error("error spawning acp agent process: {0}")]
    SpawnProc(io::Error),
    #[error("there was an internal error")]
    Internal,
    #[error("error initializing agent: {0}")]
    Init(RpcSendErr),
    #[error("agent's acp version `{0:?}` is not supported")]
    UnsupportedAcpVersion(AcpProtocolVersion),
}
