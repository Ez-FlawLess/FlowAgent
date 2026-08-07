use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};

use crate::{
    json_rpc::{id::JsonRpcIdHandler, request::RpcRequest, version::JsonRpcVersion},
    requests::Request,
};

mod id;
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
}

impl<W, R> JsonRpc<W, R>
where
    W: AsyncWriteExt + Unpin,
{
    pub async fn send_request<Re: Request>(&mut self, request: Re) {
        let body = RpcRequest {
            version: JsonRpcVersion::V2,
            id: self.id_handler.get_id(),
            method: Re::method(),
            params: request,
        };

        let mut payload = serde_json::to_string(&body).unwrap();
        payload.push('\n');

        self.writer.write_all(payload.as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}

impl<W, R> JsonRpc<W, R>
where
    R: AsyncRead + Unpin,
{
    pub async fn read_response(&mut self) -> String {
        let mut response = String::new();
        self.reader.read_line(&mut response).await.unwrap();
        response
    }
}
