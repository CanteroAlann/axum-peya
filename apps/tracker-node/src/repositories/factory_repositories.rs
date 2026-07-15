use std::error::Error;
use async_trait::async_trait;
use crate::repositories::geolocable_repository::GeoRepository;

#[async_trait]
pub trait RepositoryFactory {
    async fn create_leader_repository(&self) -> Result<GeoRepository, Box<dyn Error + Send + Sync>>;
    async fn create_follower_repository(&self) -> Result<GeoRepository, Box<dyn Error + Send + Sync>>;
}

pub type Factory = Box<dyn RepositoryFactory + Send + Sync>;