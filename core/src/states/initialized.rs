use crate::{
    Core,
    states::{State, sealed::Sealed},
};

pub struct Initialized {
    pub client_name: String,
}

impl Sealed for Initialized {}
impl State for Initialized {}

impl Core<Initialized> {
    pub fn client_name(&self) -> &str {
        self.state.client_name.as_str()
    }
}

