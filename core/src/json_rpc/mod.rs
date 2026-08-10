use thiserror::Error;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};

use crate::{
    json_rpc::{
        id::JsonRpcIdHandler,
        message::{RpcMessage, RpcMsgPayload},
        request::RpcRequest,
        version::JsonRpcVersion,
    },
    schemes::{Request, Response},
};

mod id;
mod message;
mod request;
pub mod version;

pub struct JsonRpc<W, R> {
    writer: W,
    reader: BufReader<R>,
    id_handler: JsonRpcIdHandler,
}

impl<W, R> JsonRpc<W, R>
where
    W: AsyncWriteExt + Unpin,
    R: AsyncRead + Unpin,
{
    pub fn new(writer: W, reader: R) -> Self {
        Self {
            writer,
            reader: BufReader::new(reader),
            id_handler: JsonRpcIdHandler::new(),
        }
    }

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

        self.writer
            .write_all(payload.as_bytes())
            .await
            .map_err(RpcSendErr::Write)?;
        self.writer.flush().await.map_err(RpcSendErr::Write)?;

        let mut response = String::new();
        self.reader
            .read_line(&mut response)
            .await
            .map_err(RpcSendErr::Read)?;

        let msg = serde_json::from_str::<RpcMessage>(response.as_str()).unwrap();
        let payload = RpcMsgPayload::try_from(msg).unwrap();

        match payload {
            RpcMsgPayload::Response { id: _, result } => {
                Ok(serde_json::from_str::<Res>(result.get()).unwrap())
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Error)]
pub enum RpcSendErr {
    #[error("there was an internal error")]
    Internal,
    #[error("failed to write request to IO: {0}")]
    Write(io::Error),
    #[error("failed to read from IO: {0}")]
    Read(io::Error),
}
