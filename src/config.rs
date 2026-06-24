use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;

pub struct Config {
    pub peer_id: u32,
    pub database_leader_url: String,
    pub database_follower_url: String,
    pub cluster_nodes: HashMap<u32, String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let peer_id = env::var("PEER_ID").expect("PEER_ID must be set").parse().expect("PEER_ID must be a number");
        let database_leader_url = env::var("DATABASE_LEADER_URL").expect("DATABASE_LEADER_URL must be set");
        let database_follower_url = env::var("DATABASE_FOLLOWER_URL").expect("DATABASE_FOLLOWER_URL must be set");
        
        // Parseamos la variable CLUSTER_NODES
        let cluster_nodes_raw = env::var("CLUSTER_NODES").expect("CLUSTER_NODES must be set");
        let mut cluster_nodes = HashMap::new();
        
        for node in cluster_nodes_raw.split(',') {
            if let Some((node_id_str, url)) = node.split_once('=') {
                let node_id = node_id_str.parse::<u32>().expect("Invalid Node ID in CLUSTER_NODES");
                cluster_nodes.insert(node_id, url.to_string());
            }
        }

        Config {
            peer_id,
            database_leader_url,
            database_follower_url,
            cluster_nodes,
        }
    }
}