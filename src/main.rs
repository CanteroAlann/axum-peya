mod web_server;
mod peer_server;
mod peer_client;
mod infrastructure;
mod repositories;
mod entities;
mod config;
mod app_state;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let (server_tx, server_rx) = mpsc::channel::<u32>(100);
    let mut client_server = peer_client::ClientServer::new(&config.cluster_nodes, config.peer_id).await;    
    let database = infrastructure::postgres_db::Database::new(&config.database_leader_url).await.unwrap();
    let app_state = Arc::new(Mutex::new(app_state::AppState::new(false))); 
    let web_server_handle = tokio::spawn(async move  {
        web_server::start_web_server(database.clone()).await;
    });
    let client_server_handle = tokio::spawn(async move {
        client_server.start(app_state.clone(), server_rx).await.unwrap();
    });
    
    let peer_server_handle = tokio::spawn(async move {
        peer_server::start_peer_server(config.peer_id,server_tx
        ).await.unwrap();
    });
    
    let _ = tokio::join!(web_server_handle, peer_server_handle,client_server_handle);
}
