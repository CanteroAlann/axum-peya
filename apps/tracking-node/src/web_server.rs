use axum::{routing::get, Router, extract::State};
use std::net::SocketAddr;
use crate::app_state::AppState;
use std::sync::{Arc, RwLock};

async fn hello() -> &'static str {
    "Hello world from axum!"
}

async fn hello_with_name(
    State(app_state): State<Arc<RwLock<AppState>>>, 
    name: String
) -> String {
    
    let db = {
        let state = app_state.read().unwrap();
        state.get_database().clone() 
    };

    match db.add_restaurant(crate::entities::Restaurant {
        id: 1,
        name: "Test Restaurant".to_string(),
        latitude: 40.7128,
        longitude: -74.0060,
    }).await {
        Ok(_) => println!("✅ Registro enviado a Redis exitosamente desde el handler."),
        Err(e) => println!("❌ ERROR REAL DE REDIS: {:?}", e),
    };

    format!("Hello, {}!", name)
}

pub async fn start_web_server(app_state: Arc<RwLock<AppState>>) {
    let app = Router::new()
        .route("/", get(hello))
        .route("/hello/:name", get(hello_with_name))
        .with_state(app_state); 

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind");
    
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.expect("server error");
}