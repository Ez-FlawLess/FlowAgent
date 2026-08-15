use serde::Serialize;

use crate::json_rpc::{id::JsonRpcId, version::JsonRpcVersion};

#[derive(Serialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: JsonRpcVersion,
    pub id: JsonRpcId,
    pub result: T,
}
