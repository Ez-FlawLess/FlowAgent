use std::sync::Arc;

use super::shared::Shared;

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    pub fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    pub async fn recv(&self) -> Option<T> {
        self.shared.get().await
    }
}
