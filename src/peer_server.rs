pub mod replicaprotocol{
    tonic::include_proto!("replica_protocol");
}
use std::net::SocketAddr;
use replicaprotocol::election_service_server::{ElectionService, ElectionServiceServer};
use replicaprotocol::{Ack, Heartbeat};
use tonic::{transport::Server, Request as TonicRequest, Response as TonicResponse, Status};
use tokio::sync::mpsc;

#[derive(Debug)]
struct PeerServer{
    id : u32,
    server_tx : mpsc::Sender<u32>,
}

#[tonic::async_trait]

impl ElectionService for PeerServer{
    async fn send_heartbeat(&self, request: TonicRequest<Heartbeat>) -> Result<TonicResponse<Ack>, Status> {
        println!("Received heart beat from {:?}", request.remote_addr());
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

pub async fn start_peer_server(id: u32, server_tx: mpsc::Sender<u32>) -> Result<(), Box<dyn std::error::Error>> {
    let peer_server = PeerServer { id, server_tx };
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));
    println!("Peer server {} listening on {}", id, addr);
    Server::builder()
        .add_service(ElectionServiceServer::new(peer_server))
        .serve(addr)
        .await?;
    Ok(())
}