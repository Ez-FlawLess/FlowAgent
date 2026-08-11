use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionId(String);

impl SessionId {
    pub fn id(&self) -> &str {
        self.0.as_str()
    }
}
