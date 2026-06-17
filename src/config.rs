use dotenvy::dotenv;
use std::env;


pub struct Config {
    pub database_leader_url: String,
    pub database_follower_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_leader_url = env::var("DATABASE_LEADER_URL").expect("DATABASE_LEADER_URL must be set");
        let database_follower_url = env::var("DATABASE_FOLLOWER_URL").expect("DATABASE_FOLLOWER_URL must be set");
        Config {
            database_leader_url,
            database_follower_url,
        }
    }
}