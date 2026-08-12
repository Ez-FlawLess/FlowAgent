use crate::{
    Core,
    schemes::session::id::SessionId,
    states::{
        State,
        initialized::Initialized,
        sealed::Sealed,
        session::config::{ConfigItem, ConfigItemOption},
    },
};

pub mod config;

pub struct Session {
    pub(crate) initialized: Initialized,
    pub(crate) session_id: SessionId,
    pub(crate) model: ConfigItem,
}

impl Sealed for Session {}
impl State for Session {}

impl Core<Session> {
    pub fn client_name(&self) -> &str {
        self.state.initialized.client_name.as_str()
    }

    pub fn session_id(&self) -> &str {
        self.state.session_id.id()
    }

    pub fn model_options(&self) -> &[ConfigItemOption] {
        &self.state.model.options
    }

    pub fn current_model(&self) -> &ConfigItemOption {
        &self.state.model.value
    }
}
