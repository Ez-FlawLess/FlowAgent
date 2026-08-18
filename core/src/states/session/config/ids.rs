use crate::schemes::session::config_option as schemas;

pub trait SessionConfigValueId:
    Clone + Eq + From<schemas::SessionConfigValueId> + Into<String>
{
}

pub mod model {
    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    pub struct ModelConfigId(String);
    impl SessionConfigValueId for ModelConfigId {}

    impl From<schemas::SessionConfigValueId> for ModelConfigId {
        fn from(value: schemas::SessionConfigValueId) -> Self {
            Self(value.0)
        }
    }

    impl From<ModelConfigId> for String {
        fn from(value: ModelConfigId) -> Self {
            value.0
        }
    }
}

pub mod mode {
    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    pub struct ModeConfigId(String);
    impl SessionConfigValueId for ModeConfigId {}

    impl From<schemas::SessionConfigValueId> for ModeConfigId {
        fn from(value: schemas::SessionConfigValueId) -> Self {
            Self(value.0)
        }
    }

    impl From<ModeConfigId> for String {
        fn from(value: ModeConfigId) -> Self {
            value.0
        }
    }
}
