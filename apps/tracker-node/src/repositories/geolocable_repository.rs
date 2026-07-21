use std::{error::Error};
use crate::entities::{Restaurant, Delivery};
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait GeocableRepository{
    async fn add_restaurant(&self, restaurant: Restaurant) -> Result<(), Box<dyn Error>>;
    async fn add_delivery(&self, delivery: Delivery) -> Result<(), Box<dyn Error>>;
    async fn get_nearby_restaurants(&self, latitude: f64, longitude: f64, radius: f64) -> Result<Vec<String>, Box<dyn Error>>;
}

pub type GeoRepository = Arc<dyn GeocableRepository + Send + Sync>;