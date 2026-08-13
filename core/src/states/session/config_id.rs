use crate::schemes::session::config_option::SessionConfigValueId;

pub trait ConfigItemValueId: Clone + Eq + From<SessionConfigValueId> + Into<String> {}

pub mod model {
    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    pub struct ModelConfigId(String);
    impl ConfigItemValueId for ModelConfigId {}

    impl From<SessionConfigValueId> for ModelConfigId {
        fn from(value: SessionConfigValueId) -> Self {
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
    impl ConfigItemValueId for ModeConfigId {}

    impl From<SessionConfigValueId> for ModeConfigId {
        fn from(value: SessionConfigValueId) -> Self {
            Self(value.0)
        }
    }

    impl From<ModeConfigId> for String {
        fn from(value: ModeConfigId) -> Self {
            value.0
        }
    }
}
