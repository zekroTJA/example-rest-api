pub mod redis;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Persistence {
    async fn set(&self, key: &str, val: &[u8], expiry: usize) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn exists(&self, key: &str) -> Result<bool>;
    async fn list(&self, key: &str) -> Result<Vec<(String, Vec<u8>)>>;
    async fn delete(&self, key: &[&str]) -> Result<()>;
}
