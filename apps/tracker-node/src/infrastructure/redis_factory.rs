use crate::repositories::factory_repositories::RepositoryFactory;
use crate::repositories::geolocable_repository::GeoRepository;
use crate::infrastructure::redis_db::RedisDatabase;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;

pub struct RedisFactory {
    master_url: String,
    replica_url: String,
}

impl RedisFactory {
    pub fn new(master_url: String, replica_url: String) -> Self {
        Self { master_url, replica_url }
    }
}

#[async_trait]
impl RepositoryFactory for RedisFactory {
    async fn create_leader_repository(&self) -> Result<GeoRepository, Box<dyn Error + Send + Sync>> {
        let client = redis::Client::open(self.master_url.as_str())?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        let db = RedisDatabase::new(conn);
        Ok(Arc::new(db))
    }

    async fn create_follower_repository(&self) -> Result<GeoRepository, Box<dyn Error + Send + Sync>> {
        let client = redis::Client::open(self.replica_url.as_str())?;
        let conn = client.get_multiplexed_tokio_connection().await?;
        let db = RedisDatabase::new(conn);
        Ok(Arc::new(db))
    }
}