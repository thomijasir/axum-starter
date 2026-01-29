use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
#[derive(Clone, Debug)]
pub struct CacheEntry {
    data: Value,
    expires: Instant,
}
#[derive(Clone, Debug)]
pub struct Cache {
    store: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}
impl Cache {
    pub fn new(ttl: Duration) -> Self {
        Cache {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }
    pub async fn set(&self, key: String, value: Value) {
        let mut store = self.store.write().await;
        store.insert(
            key,
            CacheEntry {
                data: value,
                expires: Instant::now() + self.ttl,
            },
        );
    }
    pub async fn get(&self, key: &str) -> Option<Value> {
        let store = self.store.read().await;
        if let Some(entry) = store.get(key) {
            if entry.expires > Instant::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }
    pub async fn delete(&self, key: &str) {
        let mut store = self.store.write().await;
        store.remove(key);
    }
    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        store.clear();
    }
}
