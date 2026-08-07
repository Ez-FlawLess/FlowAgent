use derive_new::new;
use tokio::io::AsyncWriteExt;

use crate::{
    json_rpc::{request::RpcRequest, version::JsonRpcVersion},
    requests::Request,
};

mod request;
pub mod version;

#[derive(new)]
pub struct JsonRpc<W: AsyncWriteExt + Unpin> {
    writer: W,
}

impl<W> JsonRpc<W>
where
    W: AsyncWriteExt + Unpin,
{
    pub async fn send_request<R: Request>(&mut self, request: R) {
        let body = RpcRequest {
            version: JsonRpcVersion::V2,
            id: 0,
            method: R::method(),
            params: request,
        };

        let mut payload = serde_json::to_string(&body).unwrap();
        payload.push('\n');

        self.writer.write_all(payload.as_bytes()).await.unwrap();
        self.writer.flush().await.unwrap();
    }
}
