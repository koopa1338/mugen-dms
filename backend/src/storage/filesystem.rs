use std::path::PathBuf;
use tokio::fs;

use crate::error::ApiError;
use super::StorageBackend;

/// Stores files on the local filesystem under `{root}/{bucket}/{key}`.
#[derive(Debug, Clone)]
pub struct FilesystemStorage {
    root: PathBuf,
}

impl FilesystemStorage {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn resolve(&self, bucket: &str, key: &str) -> PathBuf {
        self.root.join(bucket).join(key)
    }
}

impl StorageBackend for FilesystemStorage {
    async fn store(&self, bucket: &str, key: &str, data: &[u8]) -> Result<(), ApiError> {
        let path = self.resolve(bucket, key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&path, data).await?;
        Ok(())
    }

    async fn retrieve(&self, bucket: &str, key: &str) -> Result<Vec<u8>, ApiError> {
        let path = self.resolve(bucket, key);
        let data = fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ApiError::NotFound(format!("File not found: {bucket}/{key}"))
            } else {
                ApiError::from(e)
            }
        })?;
        Ok(data)
    }

    async fn delete(&self, bucket: &str, key: &str) -> Result<(), ApiError> {
        let path = self.resolve(bucket, key);
        fs::remove_file(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ApiError::NotFound(format!("File not found: {bucket}/{key}"))
            } else {
                ApiError::from(e)
            }
        })?;
        Ok(())
    }

    async fn exists(&self, bucket: &str, key: &str) -> Result<bool, ApiError> {
        let path = self.resolve(bucket, key);
        Ok(fs::try_exists(&path).await?)
    }
}
