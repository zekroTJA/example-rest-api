pub mod collections;
pub mod errors;
pub mod objects;
mod util;

use crate::persistence::{redis::RedisClient, Persistence};
use anyhow::Result;
use std::env;

pub struct Controller {
    object_lifetime: usize,
    persistence: Box<dyn Persistence>,
}

impl Controller {
    pub fn new(persistence: Box<dyn Persistence>, session_lifetime: usize) -> Self {
        Self {
            object_lifetime: session_lifetime,
            persistence,
        }
    }

    pub fn from_env() -> Result<Self> {
        let object_lifetime: usize = get_env("OBJECT_LIFETIME_SECS").and_then(|v| {
            v.parse()
                .map_err(|err| anyhow::anyhow!("Failed parsing OBJECT_LIFETIME_SECS: {err}"))
        })?;

        let redis_url = get_env("PERSISTENCE_REDIS")?;

        let persistence = RedisClient::new(&redis_url)?;

        Ok(Self::new(Box::new(persistence), object_lifetime))
    }
}

fn get_env(key: &str) -> Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Environment variable {key} is not set"))
}
