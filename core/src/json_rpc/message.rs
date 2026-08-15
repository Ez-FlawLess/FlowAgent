use serde::Deserialize;
use serde_json::value::RawValue;
use thiserror::Error;

use super::id::JsonRpcId;
use super::version::JsonRpcVersion;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcMessage {
    #[serde(rename = "jsonrpc")]
    pub _version: JsonRpcVersion,
    #[serde(default)]
    pub id: Option<JsonRpcId>,
    #[serde(default)]
    pub result: Option<Box<RawValue>>,
    #[serde(default)]
    pub error: Option<Box<RawValue>>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub params: Option<Box<RawValue>>,
}

#[cfg_attr(test, derive(Debug))]
pub enum RpcMsgPayload {
    Response {
        id: JsonRpcId,
        result: Box<RawValue>,
    },
    Error {
        id: JsonRpcId,
        error: Box<RawValue>,
    },
    Request {
        id: JsonRpcId,
        method: String,
        params: Box<RawValue>,
    },
    Notification {
        method: String,
        params: Box<RawValue>,
    },
}

impl TryFrom<RpcMessage> for RpcMsgPayload {
    type Error = InvalidRpcMsg;

    fn try_from(value: RpcMessage) -> Result<Self, InvalidRpcMsg> {
        match value {
            RpcMessage {
                id: Some(id),
                error: Some(error),
                ..
            } => Ok(Self::Error { id, error }),
            RpcMessage {
                id: Some(id),
                result: Some(result),
                ..
            } => Ok(Self::Response { id, result }),
            RpcMessage {
                id: Some(id),
                method: Some(method),
                params: Some(params),
                ..
            } => Ok(Self::Request { id, method, params }),
            RpcMessage {
                id: None,
                method: Some(method),
                params: Some(params),
                ..
            } => Ok(Self::Notification { method, params }),
            _ => Err(InvalidRpcMsg),
        }
    }
}

#[derive(Debug, Error)]
#[error("rpc message is invalid")]
pub struct InvalidRpcMsg;
