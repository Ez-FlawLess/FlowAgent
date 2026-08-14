use thiserror::Error;

use crate::{
    Core,
    acp_agent::AcpAgent,
    json_rpc::{JsonRpc, RpcSendErr},
    schemes::init::{ClientInfo, InitReq, InitRes},
    shared::acp_protocl_version::AcpProtocolVersion,
    states::{State, initialized::Initialized, sealed::Sealed},
};

pub struct Created;

impl Sealed for Created {}
impl State for Created {}

impl<A: AcpAgent> Core<A, Created> {
    pub fn new(acp_agent: A) -> Self {
        let (reader, writer, process) = acp_agent.into_parts();
        let rpc = JsonRpc::new(writer, reader);

        Self {
            acp_process: process,
            rpc,
            state: Created,
        }
    }

    pub async fn initialize(self) -> Result<Core<A, Initialized>, InitializeErr> {
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
            .await?;

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
pub enum InitializeErr {
    #[error("agent returned error: {0}")]
    Rpc(#[from] RpcSendErr),
    #[error("agent's acp version `{0:?}` is not supported")]
    UnsupportedAcpVersion(AcpProtocolVersion),
}
