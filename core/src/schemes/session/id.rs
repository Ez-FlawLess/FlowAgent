use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SessionId(String);

impl SessionId {
    pub fn id(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(test)]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}
