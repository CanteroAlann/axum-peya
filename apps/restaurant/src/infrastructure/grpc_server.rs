// ./apps/grpc-proxy/src/main.rs
pub mod proto {
    pub mod tracker {
        tonic::include_proto!("tracker");
    }
}

use proto::tracker::tracking_service_server::{TrackingService, TrackingServiceServer};
use proto::tracker::{
    RestaurantLocationRequest, GenericResponse,
     NearbyRequest, NearbyResponse};
use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use crate::services::sender::Sender;


#[derive(Clone)]
struct GrpcServer {
    client: Arc<RwLock<Sender>>,
}

#[tonic::async_trait]
impl TrackingService for GrpcServer {
    async fn add_restaurant_location(
        &self,
        request: Request<RestaurantLocationRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        println!("Routing write request to leader node...");
        let client = {self.client.read().await};
        let restaurant_data = request.into_inner();
        let restaurant= crate::entities::Restaurant {
            id: restaurant_data.id,
            name: restaurant_data.name,
            latitude: restaurant_data.latitude,
            longitude: restaurant_data.longitude,
        };

        client.send_restaurant_location(restaurant).await.unwrap();
        Ok(Response::new(GenericResponse {
            success: true,
            message: "Restaurant location added successfully".into(),
        }))
    }

    // RUTA DE LECTURA -> Se puede balancear (aquí elegimos el primero de la lista para iniciar)
    async fn get_nearby_restaurants(
        &self,
        request: Request<NearbyRequest>,
    ) -> Result<Response<NearbyResponse>, Status> {
        // Aquí se implementaría la lógica para obtener restaurantes cercanos
        // Por simplicidad, devolvemos una respuesta vacía
        Ok(Response::new(NearbyResponse {
            restaurant_ids: vec![],
        }))
    }
}




    pub async fn start_server(sender: Sender) -> Result<(), Box<dyn std::error::Error>> {

    let grpc_server = GrpcServer {
        client: Arc::new(RwLock::new(sender)),
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Restaurant service corriendo en el puerto 3000...");

    tonic::transport::Server::builder()
        .add_service(TrackingServiceServer::new(grpc_server))
        .serve(addr)
        .await?;

    Ok(())
}
