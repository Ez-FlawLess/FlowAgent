use serde_json::value::RawValue;
use tokio::sync::oneshot::Sender;

use crate::{json_rpc::id::JsonRpcId, utils::vec_ext::VecExt};

pub struct WaitingList {
    ids: Vec<JsonRpcId>,
    senders: Vec<Sender<Box<RawValue>>>,
}

pub struct WaitingListIndex(usize);

impl WaitingList {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            senders: Vec::new(),
        }
    }

    pub fn add_to_list(&mut self, id: JsonRpcId, sender: Sender<Box<RawValue>>) {
        self.ids.push(id);
        self.senders.push(sender);
    }

    pub fn find_index(&self, id: JsonRpcId) -> Option<WaitingListIndex> {
        self.ids
            .iter()
            .position(|list_id| *list_id == id)
            .map(|index| WaitingListIndex(index))
    }

    pub fn get_sender(&mut self, index: WaitingListIndex) -> Sender<Box<RawValue>> {
        // # Safety
        //
        // WaitingListIndex can only be created from `Self::find_index` so the index is valid
        unsafe {
            let _ = self.ids.swap_remove_unchecked(index.0);
            self.senders.swap_remove_unchecked(index.0)
        }
    }
}

// AI generated
#[cfg(test)]
mod tests {
    use serde_json::value::RawValue;

    use crate::json_rpc::id::JsonRpcIdHandler;

    use super::WaitingList;

    fn make_id(handler: &JsonRpcIdHandler) -> crate::json_rpc::id::JsonRpcId {
        handler.get_id()
    }

    #[test]
    fn add_to_list_then_find_index() {
        let handler = JsonRpcIdHandler::new();
        let mut list = WaitingList::new();

        let id1 = make_id(&handler);
        let id2 = make_id(&handler);

        let (_tx1, _rx1) = tokio::sync::oneshot::channel::<Box<RawValue>>();
        let (_tx2, _rx2) = tokio::sync::oneshot::channel::<Box<RawValue>>();

        list.add_to_list(id1.clone(), _tx1);
        list.add_to_list(id2.clone(), _tx2);

        let idx1 = list.find_index(id1.clone()).expect("id1 should be found");
        let idx2 = list.find_index(id2.clone()).expect("id2 should be found");
        assert_ne!(idx1.0, idx2.0);

        // unknown id is not found
        let unknown = make_id(&handler);
        assert!(list.find_index(unknown).is_none());
    }

    #[tokio::test]
    async fn get_sender_delivers_message() {
        let handler = JsonRpcIdHandler::new();
        let mut list = WaitingList::new();

        let id1 = make_id(&handler);
        let id2 = make_id(&handler);

        let (tx1, rx1) = tokio::sync::oneshot::channel::<Box<RawValue>>();
        let (_tx2, _rx2) = tokio::sync::oneshot::channel::<Box<RawValue>>();

        list.add_to_list(id1.clone(), tx1);
        list.add_to_list(id2.clone(), _tx2);

        // retrieving one entry removes it and leaves the other intact
        let idx1 = list.find_index(id1.clone()).unwrap();
        let sender = list.get_sender(idx1);

        sender
            .send(RawValue::from_string(String::from(r#"{"result":1}"#)).unwrap())
            .unwrap();
        let received = rx1.await.unwrap();
        assert_eq!(received.get(), r#"{"result":1}"#);

        // id1 should now no longer be found
        assert!(list.find_index(id1.clone()).is_none());
        // id2 should still be present and retrievable
        let idx2 = list.find_index(id2.clone()).unwrap();
        let _sender2 = list.get_sender(idx2);
    }
}
