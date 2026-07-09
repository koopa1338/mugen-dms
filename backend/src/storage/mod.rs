pub mod filesystem;

pub use filesystem::FilesystemStorage;

use std::future::Future;
use crate::error::ApiError;

/// Pluggable storage backend for document files.
///
/// `bucket` is a logical namespace (typically the username).
/// `key` is a relative path within that bucket (e.g. "uuid-filename.pdf").
///
/// Swap in an S3Storage implementation (using `aws-sdk-s3` or `object_store`)
/// without changing any call sites.
pub trait StorageBackend: Send + Sync {
    fn store(&self, bucket: &str, key: &str, data: &[u8]) -> impl Future<Output = Result<(), ApiError>> + Send;
    fn retrieve(&self, bucket: &str, key: &str) -> impl Future<Output = Result<Vec<u8>, ApiError>> + Send;
    fn delete(&self, bucket: &str, key: &str) -> impl Future<Output = Result<(), ApiError>> + Send;
    fn exists(&self, bucket: &str, key: &str) -> impl Future<Output = Result<bool, ApiError>> + Send;
}
