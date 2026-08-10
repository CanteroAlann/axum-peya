use std::error::Error;
use crate::entities::Restaurant;

#[async_trait::async_trait]
pub trait Repository{
    async fn new_restaurant(&self, restaurant: Restaurant) -> Result<(), Box<dyn Error>>;
}