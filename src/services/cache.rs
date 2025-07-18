use moka::sync::Cache;
use std::time::Duration;

pub struct PriceCache {
    inner: Cache<String, f64>,
}

impl PriceCache {
    pub fn new(ttl_sec: u64) -> Self {
        Self {
            inner: Cache::builder()
                .time_to_live(Duration::from_secs(ttl_sec))
                .build(),
        }
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.inner.get(key)
    }

    pub fn set(&self, key: String, value: f64) {
        self.inner.insert(key, value);
    }
}
