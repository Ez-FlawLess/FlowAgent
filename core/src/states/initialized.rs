use std::path::PathBuf;

use thiserror::Error;

use crate::{
    Core,
    json_rpc::RpcSendErr,
    schemes::session::new::{NewSessionReq, NewSessionRes},
    states::{State, sealed::Sealed, session::Session},
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

    pub async fn create_session(
        self,
        cwd: impl Into<PathBuf>,
    ) -> Result<Core<Session>, CreateSessionErr> {
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
}
