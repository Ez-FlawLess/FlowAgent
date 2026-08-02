use std::collections::HashMap;

use getset::Getters;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use thiserror::Error;

use crate::{agent::Agent, command::Command, skill::Skill, step::Step};

pub mod agent;
pub mod command;
pub mod skill;
pub mod step;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UnverifiedFlow {
    coding_agents: Vec<CodingAgent>,

    agents: Option<HashMap<String, Agent>>,

    commands: Option<HashMap<String, Command>>,

    skills: Option<Vec<Skill>>,

    /// steps of the flow
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Getters)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct VerifiedFlow {
    #[getset(get)]
    coding_agents: Vec<CodingAgent>,

    #[getset(get)]
    agents: HashMap<String, Agent>,

    #[getset(get)]
    commands: HashMap<String, Command>,

    #[getset(get)]
    skills: Vec<Skill>,

    /// steps of the flow
    #[getset(get)]
    steps: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, EnumCount)]
pub enum CodingAgent {
    OpenCode,
}

impl TryFrom<UnverifiedFlow> for VerifiedFlow {
    type Error = VerifyFlowErr;

    fn try_from(value: UnverifiedFlow) -> Result<Self, Self::Error> {
        if value.steps.is_empty() {
            return Err(VerifyFlowErr::NoSteps);
        }

        if value.coding_agents.is_empty() {
            return Err(VerifyFlowErr::NoCodeAgents);
        }

        if value.coding_agents.len() > CodingAgent::COUNT {
            return Err(VerifyFlowErr::DupCodeAgent);
        }

        {
            // list is super short so this is no problem
            // using hashset results in mem alloc and hashing overhead
            let agents = &value.coding_agents;
            if (1..agents.len()).any(|i| agents[i..].contains(&agents[i - 1])) {
                return Err(VerifyFlowErr::DupCodeAgent);
            }
        };

        for coding_agent in value.coding_agents.iter() {
            match coding_agent {
                CodingAgent::OpenCode => {
                    for step in value.steps.iter() {
                        if matches!(step.agent.as_str(), "plan" | "build") {
                            continue;
                        }
                        if value
                            .agents
                            .as_ref()
                            .and_then(|agents| agents.get(step.agent.as_str()))
                            .is_some()
                        {
                            continue;
                        }

                        return Err(VerifyFlowErr::InvalidStepAgent(step.agent.clone()));
                    }
                }
            }
        }

        Ok(Self {
            coding_agents: value.coding_agents,
            agents: value.agents.unwrap_or_default(),
            commands: value.commands.unwrap_or_default(),
            skills: value.skills.unwrap_or_default(),
            steps: value.steps,
        })
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum VerifyFlowErr {
    #[error("flow must have at least 1 coding agent")]
    NoCodeAgents,
    #[error("flow must have at least 1 step")]
    NoSteps,
    #[error("there was a duplicate coding agent in coding_agents list")]
    DupCodeAgent,
    #[error("agent '{0}' is not a default agent and not listed in the agents field")]
    InvalidStepAgent(String),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        agent::Agent, step::Step, CodingAgent, UnverifiedFlow, VerifiedFlow, VerifyFlowErr,
    };

    #[test]
    fn test_verify_flow_missing_steps() {
        let flow = UnverifiedFlow {
            coding_agents: vec![CodingAgent::OpenCode],
            steps: Vec::new(),
            ..Default::default()
        };

        let result = VerifiedFlow::try_from(flow);

        assert_eq!(result, Err(VerifyFlowErr::NoSteps));
    }

    #[test]
    fn test_verify_flow_missing_coding_agents() {
        let flow = UnverifiedFlow {
            coding_agents: Vec::new(),
            steps: vec![Step {
                agent: "plan".to_string(),
            }],
            ..Default::default()
        };

        let result = VerifiedFlow::try_from(flow);

        assert_eq!(result, Err(VerifyFlowErr::NoCodeAgents));
    }

    #[test]
    fn test_verify_flow_duplicate_coding_agents() {
        let flow = UnverifiedFlow {
            coding_agents: vec![CodingAgent::OpenCode, CodingAgent::OpenCode],
            steps: vec![Step {
                agent: "plan".to_string(),
            }],
            ..Default::default()
        };

        let result = VerifiedFlow::try_from(flow);

        assert_eq!(result, Err(VerifyFlowErr::DupCodeAgent));
    }

    #[test]
    fn test_verify_flow_invalid_step_agent() {
        let flow = UnverifiedFlow {
            coding_agents: vec![CodingAgent::OpenCode],
            steps: vec![Step {
                agent: "agi".to_string(),
            }],
            agents: Some(HashMap::from_iter([
                (
                    "coder".to_string(),
                    Agent {
                        prompt: "write code".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    "reviewer".to_string(),
                    Agent {
                        prompt: "review code".to_string(),
                        ..Default::default()
                    },
                ),
            ])),
            ..Default::default()
        };

        let result = VerifiedFlow::try_from(flow);

        assert_eq!(
            result,
            Err(VerifyFlowErr::InvalidStepAgent(String::from("agi")))
        );
    }

    #[test]
    fn test_verify_valid_flow() {
        let flow = UnverifiedFlow {
            coding_agents: vec![CodingAgent::OpenCode],
            steps: vec![Step {
                agent: "plan".to_string(),
            }],
            ..Default::default()
        };

        assert!(VerifiedFlow::try_from(flow).is_ok());

        let flow = UnverifiedFlow {
            coding_agents: vec![CodingAgent::OpenCode],
            steps: vec![
                Step {
                    agent: "reviewer".to_string(),
                },
                Step {
                    agent: "coder".to_string(),
                },
                Step {
                    agent: "plan".to_string(),
                },
                Step {
                    agent: "coder".to_string(),
                },
                Step {
                    agent: "plan".to_string(),
                },
            ],
            agents: Some(HashMap::from_iter([
                (
                    "coder".to_string(),
                    Agent {
                        prompt: "write code".to_string(),
                        ..Default::default()
                    },
                ),
                (
                    "reviewer".to_string(),
                    Agent {
                        prompt: "review code".to_string(),
                        ..Default::default()
                    },
                ),
            ])),
            ..Default::default()
        };

        assert!(VerifiedFlow::try_from(flow).is_ok());
    }
}
