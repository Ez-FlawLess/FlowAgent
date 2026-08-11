use crate::{
    Core,
    schemes::session::id::SessionId,
    states::{State, initialized::Initialized, sealed::Sealed},
};

pub struct Session {
    pub initialized: Initialized,
    pub session_id: SessionId,
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
