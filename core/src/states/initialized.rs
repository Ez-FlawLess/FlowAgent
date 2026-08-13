use std::{fmt::Display, path::PathBuf};

use thiserror::Error;

use crate::{
    Core,
    json_rpc::RpcSendErr,
    schemes::session::{
        config_option::SessionConfigOptionCategory,
        new::{NewSessionReq, NewSessionRes},
    },
    states::{
        State,
        sealed::Sealed,
        session::{
            Session,
            config::{ConfigItem, NewConfigItemErr},
        },
    },
};

pub struct Initialized {
    pub(crate) client_name: String,
}

impl Sealed for Initialized {}
impl State for Initialized {}

impl Core<Initialized> {
    pub fn client_name(&self) -> &str {
        self.state.client_name.as_str()
    }

    pub async fn create_session(
        self,
        cwd: impl Into<PathBuf>,
    ) -> Result<Core<Session>, CreateSessionErr> {
        let response = self
            .rpc
            .send::<_, NewSessionRes>(NewSessionReq {
                cwd: cwd.into(),
                mcp_servers: Vec::new(),
            })
            .await?;

        if response.session_id.id().is_empty() {
            return Err(CreateSessionErr::EmptyId);
        }

        let mut model = None;
        let mut mode = None;

        for config_option in response.config_options {
            match config_option.category {
                Some(SessionConfigOptionCategory::Model) => {
                    if model.is_some() {
                        return Err(CreateSessionErr::DupConf(CreateSessionConfs::Model));
                    }

                    model = Some(ConfigItem::new(config_option).map_err(|err| {
                        CreateSessionErr::ParseConf(CreateSessionConfs::Model, err)
                    })?);
                }
                Some(SessionConfigOptionCategory::Mode) => {
                    if mode.is_some() {
                        return Err(CreateSessionErr::DupConf(CreateSessionConfs::Mode));
                    }

                    mode = Some(ConfigItem::new(config_option).map_err(|err| {
                        CreateSessionErr::ParseConf(CreateSessionConfs::Mode, err)
                    })?);
                }
                _ => {}
            }
        }

        Ok(Core {
            acp_process: self.acp_process,
            rpc: self.rpc,
            state: Session {
                initialized: self.state,
                session_id: response.session_id,
                model: model.ok_or(CreateSessionErr::ConfMissing(CreateSessionConfs::Model))?,
                mode: mode.ok_or(CreateSessionErr::ConfMissing(CreateSessionConfs::Mode))?,
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum CreateSessionErr {
    #[error("agent returned error: {0}")]
    Rpc(#[from] RpcSendErr),
    #[error("session id returned was empty")]
    EmptyId,
    #[error("{0} config is missing")]
    ConfMissing(CreateSessionConfs),
    #[error("duplicate {0} config was provided")]
    DupConf(CreateSessionConfs),
    #[error("error parsing {0} config: {0}")]
    ParseConf(CreateSessionConfs, NewConfigItemErr),
}

#[derive(Debug)]
pub enum CreateSessionConfs {
    Model,
    Mode,
}

impl Display for CreateSessionConfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model => write!(f, "model"),
            Self::Mode => write!(f, "mode"),
        }
    }
}
