use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Agent {
    /// Name of the agent
    pub(super) name: String,
    /// System prompt for the agent
    pub(super) prompt: String,
}
