use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Step {
    /// Name of the agent used for this step
    pub(super) agent: String,
}
