use serde::{Serialize, de::DeserializeOwned};

pub mod init;
pub mod mcp;
pub mod session;

pub trait Request: Serialize {
    fn method() -> &'static str;
}

pub trait Response: DeserializeOwned {}
