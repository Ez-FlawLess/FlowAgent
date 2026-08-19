use std::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::Mutex;

pub struct Shared<T> {
    data: Mutex<Option<T>>,
    new_value: AtomicBool,
}

pub struct GetToken;

impl<T> Shared<T> {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(None),
            new_value: AtomicBool::new(false),
        }
    }

    #[inline(always)]
    pub async fn update(&self, value: T) {
        let mut data = self.data.lock().await;
        *data = Some(value);
        self.new_value.store(true, Ordering::Release);
        drop(data);
    }

    #[inline(always)]
    pub async fn get(&self) -> Option<T> {
        if self.new_value.swap(false, Ordering::Acquire) {
            let mut data = self.data.lock().await;
            data.take()
        } else {
            None
        }
    }
}
