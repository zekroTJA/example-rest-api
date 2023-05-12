use anyhow::Result;
use async_trait::async_trait;
use redis::{AsyncCommands, Client};

use super::Persistence;

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(uri: &str) -> Result<Self> {
        let client = Client::open(uri)?;

        Ok(Self { client })
    }
}

#[async_trait]
impl Persistence for RedisClient {
    async fn set(&self, key: &str, val: &[u8], expiry: usize) -> Result<()> {
        let mut con = self.client.get_async_connection().await?;
        con.set_ex(key, val, expiry).await?;
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut con = self.client.get_async_connection().await?;
        Ok(con.get(key).await?)
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let mut con = self.client.get_async_connection().await?;
        Ok(con.exists(key).await?)
    }

    async fn list(&self, key: &str) -> Result<Vec<(String, Vec<u8>)>> {
        let mut con = self.client.get_async_connection().await?;
        let keys: Vec<String> = con.keys(key).await?;

        let mut res = Vec::with_capacity(keys.len());
        for k in keys {
            let val: Option<Vec<u8>> = con.get(&k).await?;
            if let Some(val) = val {
                res.push((k, val));
            }
        }

        Ok(res)
    }

    async fn delete(&self, keys: &[&str]) -> Result<()> {
        let mut con = self.client.get_async_connection().await?;
        con.del(keys).await?;
        Ok(())
    }
}
