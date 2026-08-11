use std::{marker::PhantomData, sync::Arc};

use serde_json::value::RawValue;
use thiserror::Error;
use tokio::{
    io::{self, AsyncRead, AsyncWriteExt},
    sync::{Mutex, RwLock, oneshot},
    task::JoinHandle,
};

use crate::{
    json_rpc::{
        error::RpcError,
        id::JsonRpcIdHandler,
        listener::RpcListener,
        request::RpcRequest,
        version::JsonRpcVersion,
        waiting_list::{WaitingList, WaitingListSenderItem},
    },
    schemes::{Request, Response},
};

pub mod error;
mod id;
mod listener;
mod message;
mod request;
pub mod version;
mod waiting_list;

pub struct JsonRpc<W, R> {
    writer: Mutex<W>,
    id_handler: JsonRpcIdHandler,
    waiting_list: Arc<RwLock<WaitingList>>,
    listener_handler: JoinHandle<()>,
    _reader: PhantomData<R>,
}

impl<W, R> JsonRpc<W, R>
where
    W: AsyncWriteExt + Unpin,
    R: AsyncRead + Unpin + Send + Sync + 'static,
{
    pub fn new(writer: W, reader: R) -> Self {
        let waiting_list = Arc::new(RwLock::new(WaitingList::new()));
        let rcp_listener = RpcListener::new(reader, Arc::clone(&waiting_list));

        let handler = tokio::spawn(rcp_listener.run());

        Self {
            writer: Mutex::new(writer),
            id_handler: JsonRpcIdHandler::new(),
            waiting_list,
            listener_handler: handler,
            _reader: PhantomData,
        }
    }
}

impl<W, R> JsonRpc<W, R>
where
    W: AsyncWriteExt + Unpin,
{
    pub async fn send<Req: Request, Res: Response>(
        &mut self,
        request: Req,
    ) -> Result<Res, RpcSendErr> {
        let body = RpcRequest {
            version: JsonRpcVersion::V2,
            id: self.id_handler.get_id(),
            method: Req::method(),
            params: request,
        };

        let mut payload = serde_json::to_string(&body).or(Err(RpcSendErr::Internal))?;
        payload.push('\n');

        let (res_sender, res_receiver) = oneshot::channel::<WaitingListSenderItem>();
        {
            let mut list = self.waiting_list.write().await;
            list.add_to_list(body.id.clone(), res_sender);
        };

        {
            let mut writer = self.writer.lock().await;
            writer
                .write_all(payload.as_bytes())
                .await
                .map_err(RpcSendErr::Write)?;
            writer.flush().await.map_err(RpcSendErr::Write)?;
        };

        let response = res_receiver.await.or(Err(RpcSendErr::Internal))?;

        match response {
            Ok(response) => serde_json::from_str(response.get()).map_err(RpcSendErr::ParseRes),
            Err(error) => {
                let rpc_error =
                    serde_json::from_str::<RpcError>(error.get()).map_err(RpcSendErr::ParseRes)?;

                Err(RpcSendErr::RpcErr(rpc_error))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum RpcSendErr {
    #[error("there was an internal error")]
    Internal,
    #[error("failed to write request to IO: {0}")]
    Write(io::Error),
    #[error("failed to parse response: {0}")]
    ParseRes(serde_json::Error),
    #[error("rpc error returned")]
    RpcErr(RpcError),
}

impl<W, R> Drop for JsonRpc<W, R> {
    fn drop(&mut self) {
        self.listener_handler.abort();
    }
}
