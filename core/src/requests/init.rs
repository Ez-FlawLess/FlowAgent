use serde::Serialize;

use crate::{requests::Request, shared::acp_protocl_version::AcpProtocolVersion};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitRequest {
    #[serde(rename = "protocolVersion")]
    pub acp_protocol_version: AcpProtocolVersion,
    pub client_info: ClientInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,
    pub title: String,
    pub version: String,
}

impl Request for InitRequest {
    fn method() -> &'static str {
        "initialize"
    }
}
