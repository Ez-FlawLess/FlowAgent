use thiserror::Error;

use crate::{
    Core,
    json_rpc::RpcSendErr,
    schemes::session::{
        config_option::{
            SessionConfigId, SessionConfigOption, SessionConfigOptionVariant,
            SessionConfigSelectOption, SessionConfigSelectOptions, SessionConfigValueId,
        },
        set_config_option::{ConfigOptionPayload, SetConfigOptionReq},
    },
    states::session::Session,
};

pub(crate) struct ConfigItem {
    pub id: ConfigItemId,
    pub value: ConfigItemOption,
    pub options: Vec<ConfigItemOption>,
}

pub(crate) struct ConfigItemId(String);

#[derive(Clone, PartialEq, Eq)]
pub struct ConfigItemValueId(String);

#[derive(Clone)]
pub struct ConfigItemOption {
    id: ConfigItemValueId,
    name: String,
    description: Option<String>,
}

impl Core<Session> {
    pub fn model_options(&self) -> &[ConfigItemOption] {
        &self.state.model.options
    }

    pub fn current_model(&self) -> &ConfigItemOption {
        &self.state.model.value
    }

    pub async fn set_model(&mut self, id: ConfigItemValueId) -> Result<(), RpcSendErr> {
        self.set_config_option(SetConfigOptionReq {
            session_id: self.state.session_id.clone(),
            config_id: self.state.model.id.0.clone(),
            payload: ConfigOptionPayload::string(id.0),
        })
        .await
    }
}

impl ConfigItem {
    pub fn new(config_option: SessionConfigOption) -> Result<Self, NewConfigItemErr> {
        Self::try_from(config_option)
    }
}

impl TryFrom<SessionConfigOption> for ConfigItem {
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
            .collect::<Option<Vec<ConfigItemOption>>>()
            .ok_or(NewConfigItemErr::NotUngrouped)?;

        let current_value_id = ConfigItemValueId::from(current_value);
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

impl ConfigItemOption {
    pub fn id(&self) -> ConfigItemValueId {
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

impl From<SessionConfigValueId> for ConfigItemValueId {
    fn from(value: SessionConfigValueId) -> Self {
        Self(value.0)
    }
}

impl From<ConfigItemValueId> for SessionConfigValueId {
    fn from(value: ConfigItemValueId) -> Self {
        Self(value.0)
    }
}

impl From<SessionConfigSelectOption> for ConfigItemOption {
    fn from(value: SessionConfigSelectOption) -> Self {
        Self {
            id: value.value.into(),
            name: value.name,
            description: value.description,
        }
    }
}
