// ./apps/grpc-proxy/src/main.rs
pub mod proto {
        pub mod tracker {
            tonic::include_proto!("tracker");
        }
    }
pub mod monitor {
        tonic::include_proto!("monitor");
        }

use proto::tracker::tracking_service_server::{TrackingService, TrackingServiceServer};
use proto::tracker::tracking_service_client::TrackingServiceClient;
use monitor::monitor_service_client::MonitorServiceClient;
use monitor::Empty;
use proto::tracker::{
    RestaurantLocationRequest, GenericResponse,
     NearbyRequest, NearbyResponse};

use tonic::{transport::{Channel, Endpoint}, Request, Response, Status};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;

struct ProxyState {
    // Almacenamos los clientes ya conectados de manera permanente
    leader_client: MonitorServiceClient<Channel>,
    all_clients: Vec<MonitorServiceClient<Channel>>,
}

struct TrackerConnection {
    leader_client: TrackingServiceClient<Channel>,
    all_clients: Vec<TrackingServiceClient<Channel>>,
}

struct GrpcProxy {
    tracker: Arc<RwLock<TrackerConnection>>,
}

#[tonic::async_trait]
impl TrackingService for GrpcProxy {
    // RUTA DE ESCRITURA -> Se desvía directamente al cliente apuntado como Líder
    async fn add_restaurant_location(
        &self,
        request: Request<RestaurantLocationRequest>,
    ) -> Result<Response<GenericResponse>, Status> {
        let mut client = self.tracker.read().await.leader_client.clone();
        client.add_restaurant_location(request).await
    }

    // RUTA DE LECTURA -> Se puede balancear (aquí elegimos el primero de la lista para iniciar)
    async fn get_nearby_restaurants(
        &self,
        request: Request<NearbyRequest>,
    ) -> Result<Response<NearbyResponse>, Status> {
        let mut client = self.tracker.read().await.all_clients[0].clone();
        client.get_nearby_restaurants(request).await
    }
}

async fn monitor_cluster_roles(state: Arc<RwLock<ProxyState>>, nodes: Vec<String>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(2));
    loop {
        interval.tick().await;
        for url in &nodes {
            if let Ok(mut client) = MonitorServiceClient::connect(url.clone()).await {
                if let Ok(response) = client.check_role(Request::new(Empty {})).await {
                    let role_info = response.into_inner();
                    // Si el nodo responde que es LEADER (enum == 0), actualizamos el puntero en memoria
                    if role_info.role == 0 {
                        let mut state_write = state.write().await;
                        state_write.leader_client = client;
                        break;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node_urls = vec![
        "http://app-node-1:50051".to_string(),
        "http://app-node-2:50051".to_string(),
        "http://app-node-3:50051".to_string(),
    ];

    // Inicializamos las conexiones lazy estables de gRPC
    let mut all_clients = Vec::new();
    for url in &node_urls {
        let channel = Endpoint::from_shared(url.clone())?.connect_lazy();
        all_clients.push(MonitorServiceClient::new(channel));
    }
    let mut all_tracker_clients = Vec::new();
    for url in &node_urls {
        let channel = Endpoint::from_shared(url.clone())?.connect_lazy();
        all_tracker_clients.push(TrackingServiceClient::new(channel));
    }

    // Por defecto, asumimos temporalmente al nodo 1 como líder inicial
    let state = Arc::new(RwLock::new(ProxyState {
        leader_client: all_clients[0].clone(),
        all_clients: all_clients.clone(),
    }));
    let tracker = Arc::new(RwLock::new(TrackerConnection {
        leader_client: all_tracker_clients[0].clone(),
        all_clients: all_tracker_clients.clone(),
    }));

    // Spawneamos el hilo de fondo encargado del Smart Routing dinámico
    tokio::spawn(monitor_cluster_roles(state.clone(), node_urls));

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    println!("🚀 gRPC Reverse Proxy corriendo en el puerto 5000...");

    tonic::transport::Server::builder()
        .add_service(TrackingServiceServer::new(GrpcProxy { tracker }))
        .serve(addr)
        .await?;

    Ok(())
}
