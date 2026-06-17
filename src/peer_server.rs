pub mod replicaprotocol{
    tonic::include_proto!("replica_protocol");
}
use std::net::SocketAddr;
use replicaprotocol::heartbeat_service_server::{HeartbeatService, HeartbeatServiceServer};
use replicaprotocol::{Ack, Heartbeat};
use tonic::{transport::Server, Request as TonicRequest, Response as TonicResponse, Status};

#[derive(Debug, Default)]
struct PeerServer{
    id : u32,
}

#[tonic::async_trait]

impl HeartbeatService for PeerServer{
    async fn send_heartbeat(&self, request: TonicRequest<Heartbeat>) -> Result<TonicResponse<Ack>, Status> {
        println!("Received heart beat from {:?}", request.remote_addr());
        let response = Ack {
            message: "OK".to_string(),
        };
        Ok(TonicResponse::new(response))
    }   
}

pub async fn start_peer_server(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let peer_server = PeerServer { id };
    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));
    println!("Peer server {} listening on {}", id, addr);
    Server::builder()
        .add_service(HeartbeatServiceServer::new(peer_server))
        .serve(addr)
        .await?;
    Ok(())
}