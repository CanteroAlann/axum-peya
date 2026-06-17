mod web_server;
mod peer_server;
mod peer_client;
mod infrastructure;
mod repositories;
mod entities;
mod config;

#[tokio::main]
async fn main() {
    let config = config::Config::from_env();
    let database = infrastructure::postgres_db::Database::new(&config.database_leader_url).await.unwrap();
    let web_server_handle = tokio::spawn(async move  {
        web_server::start_web_server(database.clone()).await;
    });
    
    let peer_server_handle = tokio::spawn(async {
        peer_server::start_peer_server(1).await.unwrap();
    });
    
    let _ = tokio::join!(web_server_handle, peer_server_handle);
}
