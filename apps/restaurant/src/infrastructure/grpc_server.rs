// ./apps/grpc-proxy/src/main.rs
pub mod proto {
    pub mod restaurant {
        tonic::include_proto!("restaurant");
    }
}

use proto::restaurant::restaurant_service_server::{RestaurantService, RestaurantServiceServer};
use proto::restaurant::{
    RestaurantRequest, GenericResponse};
use tonic::{Request, Response, Status};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use crate::services::sender::Sender;


#[derive(Clone)]
struct GrpcServer {
    client: Arc<RwLock<Sender>>,
}

#[tonic::async_trait]
impl RestaurantService for GrpcServer {
    async fn new_restaurant(
        &self,
        request: Request<RestaurantRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        let client = {self.client.read().await};
        let restaurant_data = request.into_inner();
        let restaurant= crate::entities::Restaurant {
            name: restaurant_data.name,
            address: restaurant_data.address,
            password: restaurant_data.password,
            latitude: restaurant_data.latitude,
            longitude: restaurant_data.longitude,
        };

        client.send_restaurant_location(restaurant).await.unwrap();
        Ok(Response::new(GenericResponse {
            success: true,
            message: "Restaurant location added successfully".into(),
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
        .add_service(RestaurantServiceServer::new(grpc_server))
        .serve(addr)
        .await?;

    Ok(())
}
