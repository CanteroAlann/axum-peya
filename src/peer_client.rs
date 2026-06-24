pub mod replicaprotocol{
    tonic::include_proto!("replica_protocol");
}
use replicaprotocol::election_service_client::ElectionServiceClient;
use replicaprotocol::Election;
use std::sync::{Arc, Mutex};
use crate::app_state::AppState;
use tokio::sync::mpsc;

use std::collections::HashMap;
use tonic::transport::Channel;
use crate::Result;

pub struct ClientServer {
    clients: HashMap<u32, ElectionServiceClient<Channel>>,
    peer_id: u32,
    leader_id: Option<u32>,
}

impl ClientServer {
    pub async fn new(cluster_nodes: &HashMap<u32, String>, current_id: u32) -> Self {
        let mut clients = HashMap::new();

        for (&node_id, url) in cluster_nodes {
            if node_id == current_id {
                continue; 
            }

            if let Ok(endpoint) = tonic::transport::Endpoint::from_shared(url.clone()) {
                let channel = endpoint.connect_lazy();
                let client = ElectionServiceClient::new(channel);
                clients.insert(node_id, client);
            }
        }

        ClientServer   { clients, peer_id: current_id, leader_id: None }
    }

    pub async fn start(
        &mut self, 
        app_state: Arc<Mutex<AppState>>,
        mut server_rx: mpsc::Receiver<u32>

    ) -> Result<()> {
        
        let _ = self.start_election(app_state.clone()).await;

        let mut heartbeat_frecuence = tokio::time::interval(tokio::time::Duration::from_secs(3));

        loop {
            tokio::select! {
                Some(new_leader) = server_rx.recv() => {
                    println!("Client received leader update event. New leader is: {}", new_leader);
                    self.leader_id = Some(new_leader);
                    
                    let mut state = app_state.lock().unwrap();
                    if new_leader != self.peer_id {
                        state.become_follower();
                    }
                }

                _ = heartbeat_frecuence.tick() => {
                    if self.leader_id != Some(self.peer_id) {
                        if let Some(target_leader) = self.leader_id {
                            if target_leader != self.peer_id {
                                if let Err(e) = self.send_heartbeat_to_leader(target_leader).await {
                                    println!("Heartbeat failed to leader {}: {}", target_leader, e);
                                    // [ALGORITMO BULLY]: Si falla el líder, deberíamos disparar otra elección
                                    println!("Leader dead. Triggering new election...");
                                    let _ = self.start_election(app_state.clone()).await;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn start_election(&mut self, app_state: Arc<Mutex<AppState>>) -> Result<()> {
        let mut retries = 0;
        while retries < 3 {
            for (node_id, client) in &mut self.clients {
                let request = tonic::Request::new(Election {
                    replica_id: self.peer_id.to_string(),
                });
                if node_id > &self.peer_id {
                    println!("Node {} is higher than current node {}. Sending election request.", node_id, self.peer_id);
                
                    match client.send_election(request).await {
                        Ok(response) => {
                            println!("Received response from node {}: {:?}", node_id, response.into_inner().message);
                            return Ok(())
                        },  
                        Err(e) => println!("Error sending election to node {}: {:?}", node_id, e),
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            retries += 1;
        }
        self.leader_id = Some(self.peer_id);
        self.send_coordinator().await?;
        app_state.lock().unwrap().become_leader();
        println!("Node {} has become the leader.", self.peer_id);
        Ok(())
    }

    async fn send_coordinator(&mut self) -> Result<()> {
        for (node_id, client) in &mut self.clients {
            if node_id == &self.peer_id {
                continue; 
            }
            let request = tonic::Request::new(replicaprotocol::Coordinator {
                replica_id: self.peer_id.to_string(),
            });
            match client.send_coordinator(request).await {
                Ok(response) => println!("Received response from node {}: {:?}", node_id, response.into_inner().message),
                Err(e) => println!("Error sending coordinator to node {}: {:?}", node_id, e),
            }
        }
        Ok(())
    }
    async fn send_heartbeat_to_leader(&mut self, leader_id: u32) -> Result<()> {
        if let Some(client) = self.clients.get_mut(&leader_id) {
            let request = tonic::Request::new(replicaprotocol::Heartbeat {
                replica_id: self.peer_id.to_string(),
            });
            match client.send_heartbeat(request).await {
                Ok(response) => println!("Received heartbeat response from leader {}: {:?}", leader_id, response.into_inner().message),
                Err(e) => println!("Error sending heartbeat to leader {}: {:?}", leader_id, e),
            }
        } else {
            println!("Leader with ID {} not found in clients.", leader_id);
        }
        Ok(())
    }

}
