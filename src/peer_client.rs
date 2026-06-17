pub mod replicaprotocol{
    tonic::include_proto!("replica_protocol");
}
use replicaprotocol::heartbeat_service_client::HeartbeatServiceClient;
use replicaprotocol::{Heartbeat};

pub async fn send_heartbeat() -> Result<(), Box<dyn std::error::Error>> {
     let addr = "http://127.0.0.1:50051";
    let mut client = HeartbeatServiceClient::connect(addr).await?;
    let request = tonic::Request::new(Heartbeat {
        replica_id: 1.to_string(),
    });
    let response = client.send_heartbeat(request).await?;
    println!("Received response: {:?}", response.into_inner());
    Ok(())
}