use parking_lot::Mutex;
use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::watch;

pub struct AsyncDedup<K, V> {
    receivers: Mutex<HashMap<K, watch::Receiver<Option<V>>>>,
    senders: Mutex<HashMap<K, watch::Sender<Option<V>>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> AsyncDedup<K, V> {
    pub fn new() -> Self {
        Self {
            receivers: Mutex::new(HashMap::new()),
            senders: Mutex::new(HashMap::new()),
        }
    }

    pub fn take_or_create(&self, key: K) -> (watch::Receiver<Option<V>>, bool) {
        let mut receivers = self.receivers.lock();
        if let Some(existing) = receivers.get(&key) {
            (existing.clone(), false)
        } else {
            let (tx, rx) = watch::channel(None);
            receivers.insert(key.clone(), rx.clone());
            self.senders.lock().insert(key, tx);
            (rx, true)
        }
    }

    pub fn finish(&self, key: K, value: V) {
        self.receivers.lock().remove(&key);
        if let Some(tx) = self.senders.lock().remove(&key) {
            let _ = tx.send(Some(value));
        }
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for AsyncDedup<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn concurrent_requests_for_the_same_key_share_one_unit_of_work() {
        let dedup: Arc<AsyncDedup<&'static str, i32>> = Arc::new(AsyncDedup::new());
        let work_started = Arc::new(AtomicUsize::new(0));

        // Ten concurrent "requesters" for the same key.
        let mut handles = Vec::new();
        for _ in 0..10 {
            let dedup = dedup.clone();
            let work_started = work_started.clone();
            handles.push(tokio::spawn(async move {
                let (mut rx, is_first) = dedup.take_or_create("chunk");
                if is_first {
                    work_started.fetch_add(1, Ordering::SeqCst);
                    // Simulate the background generation task.
                    tokio::spawn(async move {
                        tokio::task::yield_now().await;
                        dedup.finish("chunk", 42);
                    });
                }
                loop {
                    if let Some(v) = *rx.borrow_and_update() {
                        return v;
                    }
                    rx.changed().await.unwrap();
                }
            }));
        }

        for handle in handles {
            assert_eq!(handle.await.unwrap(), 42);
        }
        assert_eq!(
            work_started.load(Ordering::SeqCst),
            1,
            "only the first concurrent request for a key should spawn the work"
        );
    }

    #[tokio::test]
    async fn a_key_can_be_requested_again_after_finishing() {
        let dedup: AsyncDedup<&'static str, i32> = AsyncDedup::new();

        let (_rx, is_first) = dedup.take_or_create("chunk");
        assert!(is_first);
        dedup.finish("chunk", 1);

        // Once finished, a fresh request for the same key is new work again, not a
        // leftover dedup entry from before.
        let (mut rx, is_first_again) = dedup.take_or_create("chunk");
        assert!(is_first_again);
        dedup.finish("chunk", 2);
        assert_eq!(*rx.borrow_and_update(), Some(2));
    }
}
