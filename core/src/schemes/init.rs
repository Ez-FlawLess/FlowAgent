use serde::{Deserialize, Serialize};

use crate::{
    schemes::{Request, Response},
    shared::acp_protocl_version::AcpProtocolVersion,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitReq {
    #[serde(rename = "protocolVersion")]
    pub acp_protocol_version: AcpProtocolVersion,
    pub client_info: ClientInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,
    pub title: Option<String>,
    pub version: String,
}

pub type AgentInfo = ClientInfo;

impl Request for InitReq {
    fn method() -> &'static str {
        "initialize"
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitRes {
    #[serde(rename = "protocolVersion")]
    pub acp_protocol_version: AcpProtocolVersion,
    pub agent_info: AgentInfo,
}

impl Response for InitRes {}
