use crate::infrastructure::postgres_db::Database;
use dotenvy::dotenv;
use sqlx::pool;
use std::collections::HashMap;
use std::env;

// The Config struct holds the configuration for the application,
// including peer ID, database URLs, and cluster nodes.
struct Config {
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

// The AppState struct holds the state of the application,
// it manages whether the current node is a leader or a follower
// updating database connection and configuration using events.

pub struct AppState {
    is_leader: bool,
    pool_to_leader: Database,
    pool_to_follower: Database,
    config: Config,
}

impl AppState {
    pub async fn new(is_leader: bool) -> Self {
        let config = Config::from_env();
        let pool_to_leader = Database::new(&config.database_leader_url).await.unwrap();
        let pool_to_follower = Database::new(&config.database_follower_url).await.unwrap();
        Self {
            is_leader: is_leader,
            pool_to_leader,
            pool_to_follower,
            config,
        }
    }
    pub fn become_leader(&mut self) {
        self.is_leader = true;
    }
    pub fn become_follower(&mut self) {
        self.is_leader = false;
    }
    pub fn get_peers_connections(&self) -> &HashMap<u32, String> {
        &self.config.cluster_nodes
    }
    pub fn get_peer_id(&self) -> u32 {
        self.config.peer_id
    }
    pub fn get_database(&self) -> &Database {
        if self.is_leader {
            &self.pool_to_leader
        } else {
            &self.pool_to_follower
        }
    }
}