use serde::{Serialize, de::DeserializeOwned};

pub mod init;

pub trait Request: Serialize {
    fn method() -> &'static str;
}

pub trait Response: DeserializeOwned {}
