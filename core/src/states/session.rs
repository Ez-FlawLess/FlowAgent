use crate::{
    Core,
    json_rpc::RpcSendErr,
    schemes::session::{
        config_option::{SessionConfigOption, SessionConfigOptionCategory},
        id::SessionId,
        set_config_option::{SetConfigOptionReq, SetConfigOptionRes},
    },
    states::{
        State,
        initialized::Initialized,
        sealed::Sealed,
        session::{config::ConfigItem, config_id::model::ModelConfigId},
    },
};

pub mod config;
pub mod config_id;

pub struct Session {
    pub(crate) initialized: Initialized,
    pub(crate) session_id: SessionId,
    pub(crate) model: ConfigItem<ModelConfigId>,
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

    fn update_configs(&mut self, config_options: Vec<SessionConfigOption>) {
        for config_option in config_options {
            if let Some(SessionConfigOptionCategory::Model) = config_option.category
                && let Ok(model) = ConfigItem::new(config_option)
            {
                self.state.model = model;
            }
        }
    }

    async fn set_config_option(&mut self, req: SetConfigOptionReq) -> Result<(), RpcSendErr> {
        let response = self.rpc.send::<_, SetConfigOptionRes>(req).await?;
        self.update_configs(response.config_options);
        Ok(())
    }
}
