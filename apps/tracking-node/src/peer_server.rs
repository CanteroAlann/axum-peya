pub mod replicaprotocol{
    tonic::include_proto!("replica_protocol");
}

pub mod monitor{
    tonic::include_proto!("monitor");
}

use std::net::SocketAddr;
use crate::app_state::AppState;
use monitor::monitor_service_server::{MonitorService, MonitorServiceServer};
use monitor::{Empty,RoleResponse};
use replicaprotocol::election_service_server::{ElectionService, ElectionServiceServer};
use replicaprotocol::{Ack, Heartbeat};
use tonic::{transport::Server, Request as TonicRequest, Response as TonicResponse, Status};
use tokio::sync::mpsc;
use std::sync::{Arc, RwLock};

struct PeerServer{
    server_tx : mpsc::Sender<u32>,
}

struct MonitorServer {
    state: Arc<RwLock<AppState>>,
}

#[tonic::async_trait]

impl ElectionService for PeerServer{
    async fn send_heartbeat(&self,_request: TonicRequest<Heartbeat>) -> Result<TonicResponse<Ack>, Status> {
        //println!("Received heart beat from {:?}", request.remote_addr());
        let response = Ack {
            message: "OK".to_string(),
        };
        Ok(TonicResponse::new(response))
    }   

    async fn send_election(&self, request: TonicRequest<replicaprotocol::Election>) -> Result<TonicResponse<Ack>, Status> {
        println!("Received election message from {:?}", request.remote_addr());
        let response = Ack {
            message: "OK".to_string(),
        };
        Ok(TonicResponse::new(response))
    }

    async fn send_coordinator(&self, request: TonicRequest<replicaprotocol::Coordinator>) -> Result<TonicResponse<Ack>, Status> {
        println!("Received coordinator message from {:?}", request.remote_addr());
        let _ = self.server_tx.send(request.into_inner().replica_id.parse().unwrap_or(0)).await;
        let response = Ack {
            message: "OK".to_string(),
        };
        Ok(TonicResponse::new(response))
    }
}

#[tonic::async_trait]

impl MonitorService for MonitorServer{
    async fn check_role(&self, request: TonicRequest<Empty>) -> Result<TonicResponse<RoleResponse>, Status> {
        println!("Received role check from {:?}", request.remote_addr());
        let response = RoleResponse {
            peer_id: self.state.read().unwrap().get_peer_id(),
            role: if self.state.read().unwrap().is_leader() {
                0 // LEADER
            } else {
                1 // FOLLOWER
            },
        };
        Ok(TonicResponse::new(response))
    }
}


pub async fn start_peer_server(app_state: Arc<RwLock<AppState>>, server_tx: mpsc::Sender<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let peer_server = PeerServer { server_tx };
    let state_for_monitor = app_state.clone();
    let monitor_server = MonitorServer { state: state_for_monitor };
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    Server::builder()
        .add_service(ElectionServiceServer::new(peer_server))
        .add_service(MonitorServiceServer::new(monitor_server))
        .serve(addr)
        .await?;
    Ok(())
}