use std::process::Stdio;

use thiserror::Error;
use tokio::{io, process::Command};

use crate::{
    Core,
    json_rpc::{JsonRpc, RpcSendErr},
    schemes::init::{ClientInfo, InitReq, InitRes},
    shared::acp_protocl_version::AcpProtocolVersion,
    states::{State, initialized::Initialized, sealed::Sealed},
};

pub struct Connected;

impl Sealed for Connected {}
impl State for Connected {}

impl Core<Connected> {
    pub async fn new() -> Result<Self, NewCoreErr> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let program = "opencode";

        #[cfg(target_os = "windows")]
        let program = "opencode.cmd";

        let mut acp_process = Command::new(program)
            .arg("acp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(NewCoreErr::SpawnProc)?;

        let stdin = acp_process.stdin.take().ok_or(NewCoreErr::Internal)?;
        let stdout = acp_process.stdout.take().ok_or(NewCoreErr::Internal)?;

        let rpc = JsonRpc::new(stdin, stdout);

        Ok(Self {
            acp_process,
            rpc,
            state: Connected,
        })
    }

    pub async fn initialize(self) -> Result<Core<Initialized>, InitializeErr> {
        let response = self
            .rpc
            .send::<_, InitRes>(InitReq {
                acp_protocol_version: AcpProtocolVersion::V1,
                client_info: ClientInfo {
                    name: "flowagent".to_string(),
                    title: Some("Flow Agent".to_string()),
                    version: "1.0.0".to_string(),
                },
            })
            .await
            .map_err(InitializeErr::Init)?;

        if response.acp_protocol_version != AcpProtocolVersion::V1 {
            return Err(InitializeErr::UnsupportedAcpVersion(
                response.acp_protocol_version,
            ));
        }

        Ok(Core {
            acp_process: self.acp_process,
            rpc: self.rpc,
            state: Initialized {
                client_name: response
                    .agent_info
                    .title
                    .unwrap_or(response.agent_info.name),
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum NewCoreErr {
    #[error("error spawning acp agent process: {0}")]
    SpawnProc(io::Error),
    #[error("there was an internal error")]
    Internal,
}

#[derive(Debug, Error)]
pub enum InitializeErr {
    #[error("error initializing agent: {0}")]
    Init(RpcSendErr),
    #[error("agent's acp version `{0:?}` is not supported")]
    UnsupportedAcpVersion(AcpProtocolVersion),
}
