pub mod proto {
    pub mod tracker {
        tonic::include_proto!("tracker");
    }
}

use proto::tracker::tracking_service_client::TrackingServiceClient;
use crate::services::sender::{Sender,Senderable};
use crate::services::sender_factory::SenderFactory;
use tonic::transport::{Channel, Endpoint};
use dotenvy::dotenv;
use async_trait::async_trait;
use crate::entities::Restaurant;

#[derive(Clone)]
pub struct GrpcClient {
    client: TrackingServiceClient<Channel>,
}

#[async_trait]
impl SenderFactory for GrpcClient {
    async fn create_sender() -> Sender {
        dotenv().ok();
        let tracker_service_url = std::env::var("TRACKER_SERVICE_URL").expect("TRACKER_SERVICE_URL must be set");
        let endpoint = Endpoint::from_shared(tracker_service_url).expect("Invalid tracker service URL");
        let client = endpoint.connect().await.expect("Failed to connect to tracker service");
        let tracker_client = TrackingServiceClient::new(client.clone());
        Box::new(GrpcClient { client: tracker_client })
    }
}

#[async_trait]
impl Senderable for GrpcClient {
    async fn send_restaurant_location(&self, message: Restaurant) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(proto::tracker::RestaurantLocationRequest {
            id: message.id,
            name: message.name,
            latitude: message.latitude,
            longitude: message.longitude,
        });
        if let Ok(_) = self.client.clone().add_restaurant_location(request).await {
            Ok(())
        } else {
            Err("Failed to send restaurant location".into())
        }
    }
}


