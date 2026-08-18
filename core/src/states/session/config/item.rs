use thiserror::Error;

use crate::{
    schemes::session::config_option as schemas, states::session::config::ids::SessionConfigValueId,
};

pub(super) struct SessionConfigItem<I: SessionConfigValueId> {
    /// id of the config item in the acp req and res
    pub config_id: schemas::SessionConfigId,
    selected_index: usize,
    options: Vec<SessionConfigItemOption<I>>,
}

pub struct SessionConfigItemOption<I: SessionConfigValueId> {
    id: I,
    name: String,
    description: Option<String>,
}

impl<I: SessionConfigValueId> SessionConfigItem<I> {
    pub fn selected(&self) -> &SessionConfigItemOption<I> {
        //  # Safety
        // selected_index is always valid
        unsafe { self.options.get_unchecked(self.selected_index) }
    }

    pub fn options(&self) -> &[SessionConfigItemOption<I>] {
        &self.options
    }
}

impl<I: SessionConfigValueId> TryFrom<schemas::SessionConfigOption> for SessionConfigItem<I> {
    type Error = NewSessionConfigItemErr;

    fn try_from(value: schemas::SessionConfigOption) -> Result<Self, Self::Error> {
        let schemas::SessionConfigOptionVariant::Select {
            current_value,
            options,
        } = value.variant
        else {
            return Err(NewSessionConfigItemErr::NotSelect);
        };

        let options = options
            .into_iter()
            .map(|option| match option {
                schemas::SessionConfigSelectOptions::Ungrouped(select_option) => {
                    Some(select_option.into())
                }
                schemas::SessionConfigSelectOptions::Grouped(_) => None,
            })
            .collect::<Option<Vec<SessionConfigItemOption<I>>>>()
            .ok_or(NewSessionConfigItemErr::NotUngrouped)?;

        let current_value_id = I::from(current_value);
        let current_value_index = options
            .iter()
            .position(|option| option.id == current_value_id)
            .ok_or(NewSessionConfigItemErr::CurrentMissing)?;

        Ok(Self {
            config_id: value.id,
            selected_index: current_value_index,
            options,
        })
    }
}

#[derive(Debug, Error)]
pub enum NewSessionConfigItemErr {
    #[error("config option variant is not `select` type")]
    NotSelect,
    #[error("select config options are not all `ungrouped` type")]
    NotUngrouped,
    #[error("current value was not in the list of options")]
    CurrentMissing,
}

impl<I: SessionConfigValueId> SessionConfigItemOption<I> {
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

impl<I: SessionConfigValueId> From<schemas::SessionConfigSelectOption>
    for SessionConfigItemOption<I>
{
    fn from(value: schemas::SessionConfigSelectOption) -> Self {
        Self {
            id: value.value.into(),
            name: value.name,
            description: value.description,
        }
    }
}
