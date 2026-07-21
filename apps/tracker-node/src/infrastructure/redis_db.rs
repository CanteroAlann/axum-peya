use crate::repositories::geolocable_repository::GeocableRepository;
use crate::entities::{Restaurant, Delivery};
use std::error::Error;
use async_trait::async_trait;

#[derive(Clone)]
pub struct RedisDatabase {
    client: redis::aio::MultiplexedConnection,
}

impl RedisDatabase {
    pub fn new(connection: redis::aio::MultiplexedConnection) -> Self {
        Self { client: connection }
    }
}
#[async_trait]
impl GeocableRepository for RedisDatabase {
    async fn add_restaurant(&self, restaurant: Restaurant) -> Result<(), Box<dyn Error>> {
        let mut conn = self.client.clone();
          let info = redis::cmd("INFO")
        .arg("replication")
        .query_async::<String>(&mut conn)
        .await;
    println!("🚀 Intentando escribir restaurant en Redis. Conexión info: {:?}", info);


        let _:() =redis::cmd("GEOADD")
            .arg("restaurants")
            .arg(restaurant.longitude)
            .arg(restaurant.latitude)
            .arg(restaurant.id.to_string())
            .query_async(&mut conn)
            .await?;
            
        Ok(())
    }

    async fn add_delivery(&self, delivery: Delivery) -> Result<(), Box<dyn Error>> {
        // Tu lógica para agregar las coordenadas del Delivery en movimiento
        Ok(())
    }

    async fn get_nearby_restaurants(
        &self,
        longitude: f64,
        latitude: f64,
        radius_km: f64,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut conn = self.client.clone();

        
        let restaurant_ids: Vec<String> = redis::cmd("GEOSEARCH")
            .arg("restaurants")
            .arg("FROMLONLAT")
            .arg(longitude)
            .arg(latitude)
            .arg("BYRADIUS")
            .arg(radius_km)
            .arg("km")
            .query_async(&mut conn)
            .await?;

        Ok(restaurant_ids)
    }
}