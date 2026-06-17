use std::error::Error;
use crate::entities::{Restaurant, Delivery};

#[axum::async_trait]
pub trait GeocableRepository{
    async fn add_restaurant(&self, restaurant: Restaurant) -> Result<(), Box<dyn Error>>;
    async fn add_delivery(&self, delivery: Delivery) -> Result<(), Box<dyn Error>>;
}