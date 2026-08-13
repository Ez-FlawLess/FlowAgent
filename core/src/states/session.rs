use crate::{
    Core,
    schemes::session::id::SessionId,
    states::{
        State,
        initialized::Initialized,
        sealed::Sealed,
        session::{
            config::ConfigItem,
            config_id::{mode::ModeConfigId, model::ModelConfigId},
        },
    },
};

pub mod config;
pub mod config_id;

pub struct Session {
    pub(crate) initialized: Initialized,
    pub(crate) session_id: SessionId,
    pub(crate) model: ConfigItem<ModelConfigId>,
    pub(crate) mode: ConfigItem<ModeConfigId>,
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
}
