use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::schemes::{
    Request, Response,
    mcp::McpServer,
    session::{config_option::SessionConfigOption, id::SessionId},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionReq {
    pub cwd: PathBuf,
    pub mcp_servers: Vec<McpServer>,
}

impl Request for NewSessionReq {
    fn method() -> &'static str {
        "session/new"
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionRes {
    pub session_id: SessionId,
    pub config_options: Vec<SessionConfigOption>,
}

impl Response for NewSessionRes {}
