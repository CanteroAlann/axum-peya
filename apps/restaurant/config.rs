use dotenvy::dotenv;


pub struct Config{
    tracker_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        let tracker_service_url = std::env::var("TRACKER_SERVICE_URL").expect("TRACKER_SERVICE_URL must be set");
        Config { tracker_service_url }
    }
    pub fn get_tracker_service_url(&self) -> &str {
        &self.tracker_service_url
    }
}

