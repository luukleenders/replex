use anyhow::Result;
use bincode::{config, Decode, Encode};
use moka::future::Cache as MokaCache;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;

// Define the maximum capacity for the global cache.
const MAX_CAPACITY: usize = 100_000;

pub static CACHE_MANAGER: Lazy<Arc<CacheManager>> = Lazy::new(|| {
    let config = Config::load();

    // Create a new CacheManager instance with the specified capacity.
    Arc::new(CacheManager::new(MAX_CAPACITY, config.cache.ttl))
});

// A wrapper around the Moka cache to provide async support and serialization.
#[derive(Debug)]
pub struct CacheManager {
    inner: MokaCache<String, Arc<Vec<u8>>>,
}

impl CacheManager {
    /// Constructs a new `CacheManager` with a specified capacity and global TTL.
    pub fn new(max_capacity: usize, global_ttl: u64) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(max_capacity.try_into().unwrap())
            .time_to_live(Duration::from_secs(global_ttl))
            .build();
        Self { inner: cache }
    }

    /// Retrieves a value from the cache, deserializing it into the desired type.
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Serialize + Debug + Decode,
    {
        let config = config::standard();

        if let Some(arc_data) = self.inner.get(key) {
            let (decoded, _len): (T, usize) =
                bincode::decode_from_slice(&arc_data[..], config).unwrap();

            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    }

    /// Inserts a value into the cache, serializing it from the provided type.
    pub async fn insert<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize + Debug + Encode,
    {
        let config = config::standard();

        let encoded: Vec<u8> = bincode::encode_to_vec(value, config).unwrap();

        // let bytes = bincode::serialize(value)?;

        self.inner.insert(key.to_owned(), Arc::new(encoded)).await;
        Ok(())
    }

    /// Deletes a value from the cache by its key.
    pub async fn delete(&self, key: &str) -> Result<()> {
        self.inner.invalidate(key).await;
        Ok(())
    }

    /// Clears all entries from the cache.
    pub async fn clear(&self) -> Result<()> {
        self.inner.invalidate_all();
        Ok(())
    }
}
