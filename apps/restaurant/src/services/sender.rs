use async_trait::async_trait;
use crate::entities::Restaurant;

#[async_trait]
pub trait Senderable {
    async fn send_restaurant_location(&self, message: Restaurant) -> Result<(), Box<dyn std::error::Error>>;
}

pub type Sender = Box<dyn Senderable + Send + Sync>;