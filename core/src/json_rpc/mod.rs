use std::{marker::PhantomData, sync::Arc, time::Duration};

use thiserror::Error;
use tokio::{
    io::{self, AsyncRead, AsyncWrite, AsyncWriteExt},
    sync::{Mutex, RwLock, oneshot},
    task::JoinHandle,
    time,
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
    W: AsyncWrite + Unpin,
    R: AsyncRead + Unpin + Send + 'static,
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
    W: AsyncWrite + Unpin,
{
    pub async fn send<Req: Request, Res: Response>(&self, request: Req) -> Result<Res, RpcSendErr> {
        self.send_with_timeout(request, Duration::from_secs(30))
            .await
    }

    pub async fn send_with_timeout<Req: Request, Res: Response>(
        &self,
        request: Req,
        timeout: Duration,
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

        let response = match time::timeout(timeout, res_receiver).await {
            Ok(Ok(res)) => res,
            Ok(Err(_)) => return Err(RpcSendErr::Internal),
            Err(_) => {
                let mut list = self.waiting_list.write().await;
                if let Some(index) = list.find_index(body.id.clone()) {
                    let _ = list.get_sender(index);
                }
                return Err(RpcSendErr::Timeout);
            }
        };

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
    #[error("request timed out")]
    Timeout,
}

impl<W, R> Drop for JsonRpc<W, R> {
    fn drop(&mut self) {
        self.listener_handler.abort();
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::value::RawValue;
    use tokio::io::{AsyncBufReadExt, BufReader};

    use crate::json_rpc::message::{RpcMessage, RpcMsgPayload};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct PingReq {
        msg: String,
    }
    impl Request for PingReq {
        fn method() -> &'static str {
            "ping"
        }
    }

    #[derive(Serialize, Deserialize)]
    struct PingRes {
        msg: String,
    }
    impl Response for PingRes {}

    #[tokio::test]
    async fn test_json_rpc_ping_request() {
        let (client_io, server_io) = io::duplex(1024);
        let (client_read, client_write) = io::split(client_io);
        let (server_read, mut server_write) = io::split(server_io);

        let rpc = JsonRpc::new(client_write, client_read);

        let server_task = tokio::spawn(async move {
            let mut reader = BufReader::new(server_read);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();

            let msg = serde_json::from_str::<RpcMessage>(line.as_str()).unwrap();

            let payload = RpcMsgPayload::try_from(msg).unwrap();

            match payload {
                RpcMsgPayload::Request { id, method, params } => {
                    assert_eq!(PingReq::method(), method);
                    let req: PingReq = serde_json::from_str(params.get()).unwrap();
                    let res_payload = PingRes { msg: req.msg };
                    let res_json = serde_json::to_string(&res_payload).unwrap();
                    let result_raw = RawValue::from_string(res_json).unwrap();

                    let response_payload = RpcMsgPayload::Response {
                        id,
                        result: result_raw,
                    };
                    let res_msg = RpcMessage::new(JsonRpcVersion::V2, response_payload);

                    let mut bytes = serde_json::to_vec(&res_msg).unwrap();
                    bytes.push(b'\n');
                    server_write.write_all(&bytes).await.unwrap();
                }
                _ => {
                    panic!("payload not Request: {:?}", payload);
                }
            };
        });

        let response: PingRes = rpc
            .send_with_timeout(
                PingReq {
                    msg: "Hi".to_string(),
                },
                Duration::from_secs(5),
            )
            .await
            .unwrap();

        assert_eq!(response.msg, "Hi");

        server_task.await.unwrap();
    }
}
