use serde::{Deserialize, Serialize};

use step::Step;

pub mod step;

#[derive(Debug, Serialize, Deserialize)]
pub struct UnverifiedFlow {
    /// steps of the flow
    steps: Vec<Step>,
}
