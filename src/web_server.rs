use axum::{routing::get, Router, extract::State};
use std::net::SocketAddr;
use std::sync::Arc;
use crate::infrastructure::postgres_db::Database;
use crate::repositories::geolocable_repository::GeocableRepository;

async fn hello() -> &'static str {
    "Hello world from axum!"
}

async fn hello_with_name(State(db): State<Arc<Database>>, name: String) -> String {
    let _ = db.add_restaurant(crate::entities::Restaurant {
        id: 1,
        name: "Test Restaurant".to_string(),
        latitude: 40.7128,
        longitude: -74.0060,
    }).await;


    format!("Hello, {}!", name)
}


pub async fn start_web_server(db : Database) {
    let database = Arc::new(db);
    let app = Router::new()
    .route("/", get(hello))
    .route("/hello/:name", get(hello_with_name))
    .with_state(database);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
    println!("Listening on {}", addr);
}