use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum McpServer {}
