use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Skill {}
