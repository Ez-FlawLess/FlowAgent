use std::path::PathBuf;

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

        for config_option in response.config_options {
            if let Some(SessionConfigOptionCategory::Model) = config_option.category {
                if model.is_some() {
                    return Err(CreateSessionErr::DupModelConf);
                }

                model =
                    Some(ConfigItem::new(config_option).map_err(CreateSessionErr::ParseModelConf)?);
            }
        }

        Ok(Core {
            acp_process: self.acp_process,
            rpc: self.rpc,
            state: Session {
                initialized: self.state,
                session_id: response.session_id,
                model: model.ok_or(CreateSessionErr::ModelConfMissing)?,
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
    #[error("model config is missing")]
    ModelConfMissing,
    #[error("duplicate model config was provided")]
    DupModelConf,
    #[error("error parsing model config: {0}")]
    ParseModelConf(NewConfigItemErr),
}
