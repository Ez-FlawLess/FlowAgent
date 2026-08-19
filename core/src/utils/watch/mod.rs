use std::sync::Arc;

pub use receiver::Receiver;
pub use sender::Sender;

use shared::Shared;

mod receiver;
mod sender;
mod shared;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::<T>::new());

    let sender = Sender::new(Arc::clone(&shared));
    let receiver = Receiver::new(shared);

    (sender, receiver)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::timeout;

    #[tokio::test]
    async fn test_watch_channel_with_one_value() {
        let (sender, receiver) = super::channel();

        let value = vec![1, 2, 3];

        assert!(receiver.recv().await.is_none());

        sender.send(value.clone()).await;

        assert_eq!(receiver.recv().await, Some(value));

        assert!(receiver.recv().await.is_none());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_watch_is_monotonic_and_final_value() {
        let (sender, receiver) = super::channel::<u32>();
        const TOTAL_MESSAGES: u32 = 100_000;

        let producer = tokio::spawn(async move {
            for i in 1..=TOTAL_MESSAGES {
                sender.send(i).await;
                if i % 128 == 0 {
                    tokio::task::yield_now().await;
                }
            }
        });

        let consumer = tokio::spawn(async move {
            let mut last_seen = loop {
                if let Some(val) = receiver.recv().await {
                    break val;
                }
            };

            while last_seen != TOTAL_MESSAGES {
                if let Some(val) = receiver.recv().await {
                    assert!(
                        val > last_seen,
                        "Received out-of-order value: {val} <= {last_seen}"
                    );
                    last_seen = val;
                } else {
                    tokio::task::yield_now().await;
                }
            }

            assert!(receiver.recv().await.is_none());
        });

        timeout(Duration::from_secs(5), consumer)
            .await
            .unwrap()
            .unwrap();
        producer.await.unwrap();
    }
}
