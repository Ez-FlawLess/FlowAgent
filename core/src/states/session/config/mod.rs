use std::fmt::Display;

use thiserror::Error;

use crate::Core;
use crate::acp_agent::AcpAgent;
use crate::json_rpc::RpcSendErr;
use crate::schemes::session::config_option as schemas;
use crate::schemes::session::set_config_option::{
    ConfigOptionPayload, SetConfigOptionReq, SetConfigOptionRes,
};
use crate::states::session::Session;
use crate::states::session::config::item::{NewSessionConfigItemErr, SessionConfigItemOption};
use crate::states::session::config::{
    ids::{mode::ModeConfigId, model::ModelConfigId},
    item::SessionConfigItem,
};

mod ids;
pub mod item;

pub struct SessionConfig {
    model: SessionConfigItem<ModelConfigId>,
    mode: SessionConfigItem<ModeConfigId>,
}

impl SessionConfig {
    pub fn new(
        config_options: Vec<schemas::SessionConfigOption>,
    ) -> Result<Self, NewSessionConfigErr> {
        let mut model = None;
        let mut mode = None;

        for config_option in config_options {
            match config_option.category {
                Some(schemas::SessionConfigOptionCategory::Model) => {
                    if model.is_some() {
                        return Err(NewSessionConfigErr::DupConf(SessionConfs::Model));
                    }

                    model =
                        Some(SessionConfigItem::try_from(config_option).map_err(|err| {
                            NewSessionConfigErr::ParseConf(SessionConfs::Model, err)
                        })?);
                }
                Some(schemas::SessionConfigOptionCategory::Mode) => {
                    if mode.is_some() {
                        return Err(NewSessionConfigErr::DupConf(SessionConfs::Mode));
                    }

                    mode =
                        Some(SessionConfigItem::try_from(config_option).map_err(|err| {
                            NewSessionConfigErr::ParseConf(SessionConfs::Mode, err)
                        })?);
                }
                _ => {}
            }
        }

        Ok(Self {
            model: model.ok_or(NewSessionConfigErr::ConfMissing(SessionConfs::Model))?,
            mode: mode.ok_or(NewSessionConfigErr::ConfMissing(SessionConfs::Mode))?,
        })
    }
}

#[derive(Debug, Error)]
pub enum NewSessionConfigErr {
    #[error("{0} config is missing")]
    ConfMissing(SessionConfs),
    #[error("duplicate {0} config was provided")]
    DupConf(SessionConfs),
    #[error("error parsing {0} config: {0}")]
    ParseConf(SessionConfs, NewSessionConfigItemErr),
}

#[derive(Debug)]
pub enum SessionConfs {
    Model,
    Mode,
}

impl Display for SessionConfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model => write!(f, "model"),
            Self::Mode => write!(f, "mode"),
        }
    }
}

impl<A: AcpAgent> Core<A, Session> {
    pub fn current_model(&self) -> &SessionConfigItemOption<ModelConfigId> {
        self.state.config.model.selected()
    }

    pub fn current_mode(&self) -> &SessionConfigItemOption<ModeConfigId> {
        self.state.config.mode.selected()
    }

    pub fn model_options(&self) -> &[SessionConfigItemOption<ModelConfigId>] {
        self.state.config.model.options()
    }

    pub fn mode_options(&self) -> &[SessionConfigItemOption<ModeConfigId>] {
        self.state.config.mode.options()
    }

    pub async fn set_model(&mut self, id: ModelConfigId) -> Result<(), SetSessionConfigErr> {
        self.set_session_config(SetConfigOptionReq {
            session_id: self.state.session_id.clone(),
            config_id: self.state.config.model.config_id.0.clone(),
            payload: ConfigOptionPayload::string(id),
        })
        .await
    }

    pub async fn set_mode(&mut self, id: ModeConfigId) -> Result<(), SetSessionConfigErr> {
        self.set_session_config(SetConfigOptionReq {
            session_id: self.state.session_id.clone(),
            config_id: self.state.config.mode.config_id.0.clone(),
            payload: ConfigOptionPayload::string(id),
        })
        .await
    }

    async fn set_session_config(
        &mut self,
        req: SetConfigOptionReq,
    ) -> Result<(), SetSessionConfigErr> {
        let response = self
            .rpc
            .send::<_, SetConfigOptionRes>(req)
            .await
            .map_err(SetSessionConfigErr::Rpc)?;
        self.state.config =
            SessionConfig::new(response.config_options).map_err(SetSessionConfigErr::Config)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum SetSessionConfigErr {
    #[error("error sending rpc request: {0}")]
    Rpc(RpcSendErr),
    #[error("failed to parse session config: {0}")]
    Config(NewSessionConfigErr),
}
