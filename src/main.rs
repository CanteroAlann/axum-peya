mod web_server;
mod peer_server;
mod peer_client;
mod infrastructure;
mod repositories;
mod entities;
mod app_state;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {

    let app_data = app_state::AppState::new(false).await;
    let (server_tx, server_rx) = mpsc::channel::<u32>(100);
    let mut client_server = peer_client::ClientServer::new(
        app_data.get_peers_connections(),
        app_data.get_peer_id()).await;
    let app_state = Arc::new(Mutex::new(app_data)); 
    let app_state_to_web = app_state.clone();
    let web_server_handle = tokio::spawn(async move  {
        web_server::start_web_server(app_state_to_web.clone()).await;
    });
    let app_state_to_client = app_state.clone();
    let client_server_handle = tokio::spawn(async move {
        client_server.start(app_state_to_client.clone(), server_rx).await.unwrap();
    });
    
    let app_state_to_peer = app_state.clone();
    let peer_server_handle = tokio::spawn(async move {
        peer_server::start_peer_server(app_state_to_peer.clone(),server_tx
        ).await.unwrap();
    });
    
    let _ = tokio::join!(web_server_handle, peer_server_handle,client_server_handle);
}
