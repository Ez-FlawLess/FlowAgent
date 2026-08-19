use std::sync::Arc;

use super::shared::Shared;

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn new(shared: Arc<Shared<T>>) -> Self {
        Self { shared }
    }

    pub async fn send(&self, value: T) {
        self.shared.update(value).await
    }
}
