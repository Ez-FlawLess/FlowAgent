use std::path::PathBuf;

use thiserror::Error;

use crate::{
    Core,
    acp_agent::AcpAgent,
    json_rpc::RpcSendErr,
    schemes::session::new::{NewSessionReq, NewSessionRes},
    states::{
        State,
        sealed::Sealed,
        session::{
            Session,
            config::{NewSessionConfigErr, SessionConfig},
        },
    },
};

pub struct Initialized {
    pub(crate) client_name: String,
}

impl Sealed for Initialized {}
impl State for Initialized {}

impl<A: AcpAgent> Core<A, Initialized> {
    pub fn client_name(&self) -> &str {
        self.state.client_name.as_str()
    }

    pub async fn create_session(
        self,
        cwd: impl Into<PathBuf>,
    ) -> Result<Core<A, Session>, CreateSessionErr> {
        let response = self
            .rpc
            .send::<_, NewSessionRes>(NewSessionReq {
                cwd: cwd.into(),
                mcp_servers: Vec::new(),
            })
            .await?;

        if response.session_id.id().is_empty() {
            return Err(CreateSessionErr::EmptyId);
        }

        Ok(Core {
            acp_process: self.acp_process,
            rpc: self.rpc,
            state: Session {
                initialized: self.state,
                session_id: response.session_id,
                config: SessionConfig::new(response.config_options)
                    .map_err(CreateSessionErr::Config)?,
            },
        })
    }
}

#[derive(Debug, Error)]
pub enum CreateSessionErr {
    #[error("agent returned error: {0}")]
    Rpc(#[from] RpcSendErr),
    #[error("session id returned was empty")]
    EmptyId,
    #[error("failed to parse session config: {0}")]
    Config(NewSessionConfigErr),
}
