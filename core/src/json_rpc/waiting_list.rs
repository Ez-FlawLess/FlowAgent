use serde_json::value::RawValue;
use tokio::sync::oneshot::Sender;

use crate::{json_rpc::id::JsonRpcId, utils::vec_ext::VecExt};

pub type WaitingListSenderItem = Result<Box<RawValue>, Box<RawValue>>;

pub struct WaitingList {
    ids: Vec<JsonRpcId>,
    senders: Vec<Sender<WaitingListSenderItem>>,
}

pub struct WaitingListIndex(usize);

impl WaitingList {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            senders: Vec::new(),
        }
    }

    pub fn add_to_list(&mut self, id: JsonRpcId, sender: Sender<WaitingListSenderItem>) {
        self.ids.push(id);
        self.senders.push(sender);
    }

    pub fn find_index(&self, id: JsonRpcId) -> Option<WaitingListIndex> {
        self.ids
            .iter()
            .position(|list_id| *list_id == id)
            .map(WaitingListIndex)
    }

    pub fn get_sender(&mut self, index: WaitingListIndex) -> Sender<WaitingListSenderItem> {
        // # Safety
        //
        // WaitingListIndex can only be created from `Self::find_index` so the index is valid
        unsafe {
            let _ = self.ids.swap_remove_unchecked(index.0);
            self.senders.swap_remove_unchecked(index.0)
        }
    }
}
