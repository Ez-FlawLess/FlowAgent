use std::sync::Arc;

use serde_json::value::RawValue;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    sync::RwLock,
};

use crate::json_rpc::{
    id::JsonRpcId,
    message::{RpcMessage, RpcMsgPayload},
    waiting_list::WaitingList,
};

pub struct RpcListener<R> {
    reader: BufReader<R>,
    waiting_list: Arc<RwLock<WaitingList>>,
}

impl<R> RpcListener<R>
where
    R: AsyncRead + Unpin,
{
    pub fn new(reader: R, waiting_list: Arc<RwLock<WaitingList>>) -> Self {
        Self {
            reader: BufReader::new(reader),
            waiting_list,
        }
    }

    pub async fn run(mut self) {
        let mut line = String::new();
        loop {
            line.clear();
            if self.reader.read_line(&mut line).await.is_err() {
                continue;
            }

            let Ok(msg) = serde_json::from_str::<RpcMessage>(line.as_str()) else {
                continue;
            };

            let Ok(payload) = RpcMsgPayload::try_from(msg) else {
                continue;
            };

            match payload {
                RpcMsgPayload::Response { id, result } => {
                    Self::handle_response(&self.waiting_list, id, Ok(result)).await
                }
                RpcMsgPayload::Error { id, error } => {
                    Self::handle_response(&self.waiting_list, id, Err(error)).await
                }
                _ => {}
            };
        }
    }

    async fn handle_response(
        waiting_list: &Arc<RwLock<WaitingList>>,
        id: JsonRpcId,
        outcome: Result<Box<RawValue>, Box<RawValue>>,
    ) {
        let index = {
            let list = waiting_list.read().await;

            match list.find_index(id) {
                Some(i) => i,
                None => return,
            }
        };

        let sender = {
            let mut list = waiting_list.write().await;
            list.get_sender(index)
        };

        let _ = sender.send(outcome);
    }
}
