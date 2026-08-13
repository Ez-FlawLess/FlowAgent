use thiserror::Error;

use crate::{
    Core,
    json_rpc::RpcSendErr,
    schemes::session::{
        config_option::{
            SessionConfigId, SessionConfigOption, SessionConfigOptionCategory,
            SessionConfigOptionVariant, SessionConfigSelectOption, SessionConfigSelectOptions,
        },
        set_config_option::{ConfigOptionPayload, SetConfigOptionReq, SetConfigOptionRes},
    },
    states::session::{
        Session,
        config_id::{ConfigItemValueId, mode::ModeConfigId, model::ModelConfigId},
    },
};

pub(crate) struct ConfigItem<I: ConfigItemValueId> {
    pub id: ConfigItemId,
    pub value: ConfigItemOption<I>,
    pub options: Vec<ConfigItemOption<I>>,
}

pub(crate) struct ConfigItemId(String);

#[derive(Clone)]
pub struct ConfigItemOption<I: ConfigItemValueId> {
    id: I,
    name: String,
    description: Option<String>,
}

impl Core<Session> {
    pub fn model_options(&self) -> &[ConfigItemOption<ModelConfigId>] {
        self.state.model.options.as_slice()
    }

    pub fn current_model(&self) -> &ConfigItemOption<ModelConfigId> {
        &self.state.model.value
    }

    pub async fn set_model(&mut self, id: ModelConfigId) -> Result<(), RpcSendErr> {
        self.set_config_option(SetConfigOptionReq {
            session_id: self.state.session_id.clone(),
            config_id: self.state.model.id.0.clone(),
            payload: ConfigOptionPayload::string(id),
        })
        .await
    }

    pub fn mode_options(&self) -> &[ConfigItemOption<ModeConfigId>] {
        self.state.mode.options.as_slice()
    }

    pub fn current_mode(&self) -> &ConfigItemOption<ModeConfigId> {
        &self.state.mode.value
    }

    pub async fn set_mode(&mut self, id: ModeConfigId) -> Result<(), RpcSendErr> {
        self.set_config_option(SetConfigOptionReq {
            session_id: self.state.session_id.clone(),
            config_id: self.state.mode.id.0.clone(),
            payload: ConfigOptionPayload::string(id),
        })
        .await
    }

    fn update_configs(&mut self, config_options: Vec<SessionConfigOption>) {
        for config_option in config_options {
            match config_option.category {
                Some(SessionConfigOptionCategory::Model) => {
                    if let Ok(model) = ConfigItem::new(config_option) {
                        self.state.model = model;
                    }
                }
                Some(SessionConfigOptionCategory::Mode) => {
                    if let Ok(mode) = ConfigItem::new(config_option) {
                        self.state.mode = mode;
                    }
                }
                _ => {}
            }
        }
    }

    async fn set_config_option(&mut self, req: SetConfigOptionReq) -> Result<(), RpcSendErr> {
        let response = self.rpc.send::<_, SetConfigOptionRes>(req).await?;
        self.update_configs(response.config_options);
        Ok(())
    }
}

impl<I: ConfigItemValueId> ConfigItem<I> {
    pub fn new(config_option: SessionConfigOption) -> Result<Self, NewConfigItemErr> {
        Self::try_from(config_option)
    }
}

impl<I: ConfigItemValueId> TryFrom<SessionConfigOption> for ConfigItem<I> {
    type Error = NewConfigItemErr;

    fn try_from(value: SessionConfigOption) -> Result<Self, Self::Error> {
        let SessionConfigOptionVariant::Select {
            current_value,
            options,
        } = value.variant
        else {
            return Err(NewConfigItemErr::NotSelectVariant);
        };

        let options = options
            .into_iter()
            .map(|option| match option {
                SessionConfigSelectOptions::Ungrouped(select_option) => Some(select_option.into()),
                SessionConfigSelectOptions::Grouped(_) => None,
            })
            .collect::<Option<Vec<ConfigItemOption<I>>>>()
            .ok_or(NewConfigItemErr::NotUngrouped)?;

        let current_value_id = I::from(current_value);
        let current_value = options
            .iter()
            .find(|option| option.id == current_value_id)
            .ok_or(NewConfigItemErr::CurrentMissing)?
            .clone();

        Ok(Self {
            id: value.id.into(),
            value: current_value,
            options,
        })
    }
}

#[derive(Debug, Error)]
pub enum NewConfigItemErr {
    #[error("config option was not of select variant")]
    NotSelectVariant,
    #[error("not all select options were of type ungrouped")]
    NotUngrouped,
    #[error("current value id missing in the options list")]
    CurrentMissing,
}

impl<I: ConfigItemValueId> ConfigItemOption<I> {
    pub fn id(&self) -> I {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl From<SessionConfigId> for ConfigItemId {
    fn from(value: SessionConfigId) -> Self {
        Self(value.0)
    }
}

impl From<ConfigItemId> for SessionConfigId {
    fn from(value: ConfigItemId) -> Self {
        Self(value.0)
    }
}

impl<I: ConfigItemValueId> From<SessionConfigSelectOption> for ConfigItemOption<I> {
    fn from(value: SessionConfigSelectOption) -> Self {
        Self {
            id: value.value.into(),
            name: value.name,
            description: value.description,
        }
    }
}
