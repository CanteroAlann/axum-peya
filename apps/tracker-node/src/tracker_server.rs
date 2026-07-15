use std::sync::{Arc, RwLock};
use std::net::SocketAddr;


pub mod tracker_server{
    tonic::include_proto!("tracker");
}

use crate::app_state::AppState;
use crate::tracker_server::tracker_server::{NearbyRequest, NearbyResponse};
use tracker_server::tracking_service_server::{TrackingService,TrackingServiceServer};
use tracker_server::{RestaurantLocationRequest,GenericResponse};
use crate::entities::Restaurant;


struct TrackerServer {
    app_state:  Arc<RwLock<AppState>>,

}

#[tonic::async_trait]

impl TrackingService for TrackerServer{
    async fn add_restaurant_location(&self, request: tonic::Request<RestaurantLocationRequest>) -> Result<tonic::Response<GenericResponse>, tonic::Status> {
        let req = request.into_inner();
        let restaurant = Restaurant::new(req.id, req.name.clone(), req.latitude, req.longitude);
        let database = {self.app_state.read().unwrap().get_database().clone()};
        database.add_restaurant(restaurant).await.map_err(|e| {
            tonic::Status::internal(format!("Failed to add restaurant location: {}", e))
        })?;
        let response = GenericResponse {
            success: true,
            message: "Location added successfully".to_string(),
        };
        Ok(tonic::Response::new(response))
    }

    async fn get_nearby_restaurants(&self, _request: tonic::Request<NearbyRequest>) -> Result<tonic::Response<tracker_server::NearbyResponse>, tonic::Status> {
        // TODO: Implement logic to retrieve nearby restaurants based on the request parameters (e.g., latitude, longitude, radius)
        let ids = Vec::new();
         // Placeholder for actual restaurant IDs
        let response = NearbyResponse {
            restaurant_ids: ids, // Placeholder for actual restaurant data
        };
        Ok(tonic::Response::new(response))
    }
}


pub async fn start_tracker_server(app_state: Arc<RwLock<AppState>>) -> Result<(), Box<dyn std::error::Error>> {
    let tracker_server = TrackerServer { app_state };
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting Tracker Server on {}", addr);

    tonic::transport::Server::builder()
        .add_service(TrackingServiceServer::new(tracker_server))
        .serve(addr)
        .await?;

    Ok(())
}