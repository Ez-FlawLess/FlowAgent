use serde::Serialize;

use crate::json_rpc::version::JsonRpcVersion;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcRequest<P: Serialize> {
    #[serde(rename = "jsonrpc")]
    pub version: JsonRpcVersion,
    pub id: u32,
    pub method: &'static str,
    pub params: P,
}
