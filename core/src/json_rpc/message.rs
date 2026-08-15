use serde::Deserialize;
use serde_json::value::RawValue;
use thiserror::Error;

#[cfg(test)]
use serde::Serialize;

use super::id::JsonRpcId;
use super::version::JsonRpcVersion;

#[derive(Deserialize)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct RpcMessage {
    #[serde(rename = "jsonrpc")]
    version: JsonRpcVersion,
    #[serde(default)]
    id: Option<JsonRpcId>,
    #[serde(default)]
    result: Option<Box<RawValue>>,
    #[serde(default)]
    error: Option<Box<RawValue>>,
    #[serde(default)]
    method: Option<String>,
    #[serde(default)]
    params: Option<Box<RawValue>>,
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

impl RpcMessage {
    pub fn new(version: JsonRpcVersion, payload: RpcMsgPayload) -> Self {
        match payload {
            RpcMsgPayload::Response { id, result } => Self {
                version,
                id: Some(id),
                result: Some(result),
                error: None,
                method: None,
                params: None,
            },
            RpcMsgPayload::Error { id, error } => Self {
                version,
                id: Some(id),
                result: None,
                error: Some(error),
                method: None,
                params: None,
            },
            RpcMsgPayload::Request { id, method, params } => Self {
                version,
                id: Some(id),
                result: None,
                error: None,
                method: Some(method),
                params: Some(params),
            },
            RpcMsgPayload::Notification { method, params } => Self {
                version,
                id: None,
                result: None,
                error: None,
                method: Some(method),
                params: Some(params),
            },
        }
    }
}
