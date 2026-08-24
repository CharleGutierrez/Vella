use std::sync::Arc;
use object_store::{ObjectStore, local::LocalFileSystem, aws::AmazonS3Builder, memory::InMemory};
use bytes::Bytes;
use object_store::path::Path as StorePath;
use tracing::info;
use crate::ai::tuner::AiTuner;

pub enum StorageConfig {
    Local(String),
    S3 {
        region: String,
        bucket: String,
        access_key: String,
        secret_key: String,
    },
    Memory,
}

#[derive(Clone)]
pub struct StorageManager {
    store: Arc<dyn ObjectStore>,
    ai_tuner: Arc<AiTuner>,
}

impl StorageManager {
    pub fn new(config: StorageConfig, ai_tuner: Arc<AiTuner>) -> Self {
        let store: Arc<dyn ObjectStore> = match config {
            StorageConfig::Local(path) => {
                info!("Initializing Local Storage at {}", path);
                std::fs::create_dir_all(&path).unwrap_or_default();
                Arc::new(LocalFileSystem::new_with_prefix(path).unwrap())
            }
            StorageConfig::S3 { region, bucket, access_key, secret_key } => {
                info!("Initializing S3 Storage for bucket {}", bucket);
                let s3 = AmazonS3Builder::new()
                    .with_region(region)
                    .with_bucket_name(bucket)
                    .with_access_key_id(access_key)
                    .with_secret_access_key(secret_key)
                    .build()
                    .unwrap();
                Arc::new(s3)
            }
            StorageConfig::Memory => {
                info!("Initializing InMemory Storage");
                Arc::new(InMemory::new())
            }
        };

        Self { store, ai_tuner }
    }

    pub async fn upload(&self, path: &str, data: Bytes) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let location = StorePath::from(path);
        self.store.put(&location, data.into()).await?;
        Ok(())
    }

    pub async fn smart_download(&self, path: &str, recent_access_count: usize) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
        let location = StorePath::from(path);
        
        let recommended_tier = self.ai_tuner.recommend_storage_tier(path, recent_access_count);
        if recommended_tier == "Memory" {
            info!("AI Tuner: Routing download request for {} via hot Memory tier", path);
            // In a full implementation, we'd check memory cache first.
        } else {
            info!("AI Tuner: Routing download request for {} via cold S3 tier", path);
        }

        let result = self.store.get(&location).await?;
        let bytes = result.bytes().await?;
        Ok(bytes)
    }
}
