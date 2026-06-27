use crate::repositories::geolocable_repository::GeocableRepository;
use std::error::Error;
use crate::entities::{Restaurant, Delivery};
use sqlx::postgres::PgPoolOptions;


#[derive(Clone)]
pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(Self { pool })
    }
}

#[axum::async_trait]
impl GeocableRepository for Database {
    async fn add_restaurant(&self, restaurant: Restaurant) -> Result<(), Box<dyn Error>> {
        let ubication = format!("POINT({} {})", restaurant.latitude, restaurant.longitude);
        let query = "INSERT INTO restaurants (id, name, ubication) VALUES ($1, $2, ST_GeomFromText($3))";
        sqlx::query(query)
            .bind(restaurant.id)
            .bind(restaurant.name)
            .bind(ubication)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn add_delivery(&self, delivery: Delivery) -> Result<(), Box<dyn Error>> {
        // Implementación para agregar una entrega a la base de datos
        Ok(())
    }
}