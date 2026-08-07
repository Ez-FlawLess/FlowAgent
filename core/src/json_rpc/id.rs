use std::sync::atomic::{AtomicU32, Ordering};

use serde::Serialize;

#[derive(PartialEq, Eq, Serialize)]
pub struct JsonRpcId(u32);

#[derive(Default)]
pub struct JsonRpcIdHandler(AtomicU32);

impl JsonRpcIdHandler {
    pub fn new() -> Self {
        Self(AtomicU32::new(0))
    }

    pub fn get_id(&self) -> JsonRpcId {
        let id = self.0.fetch_add(1, Ordering::Relaxed);
        JsonRpcId(id)
    }
}
