use crate::services::sender_factory::SenderFactory;



#[path = "../config.rs"]
mod config;
mod infrastructure;
mod services;
mod entities;
mod repositories;


#[tokio::main]
async fn main() {
    let sender = infrastructure::grpc_client::GrpcClient::create_sender().await;
    if let Err(e) = infrastructure::grpc_server::start_server(sender).await {
        eprintln!("Error starting gRPC server: {}", e);
    }
}


