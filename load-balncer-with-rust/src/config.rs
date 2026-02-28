// load-balncer-with-rust/src/config.rs

use serde::Deserialize;
use std::{fs, io};

// Root config — loaded from config.yml
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub health_check_interval: String, // e.g. "5s"
    pub listeners: Vec<ListenerConfig>,
    pub servers: Vec<ServerConfig>,
}

// Defines one listening endpoint (HTTP or TCP)
#[derive(Debug, Deserialize, Clone)]
pub struct ListenerConfig {
    pub listen_addr: String, // e.g. ":8000"
    pub mode: String,        // "http" or "tcp"
    pub algorithm: String,   // "round_robin" or "least_connections"
}

// Defines one backend server
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,           // e.g. "localhost:9001"
    pub max_connections: usize, // max allowed concurrent connections
    pub weight: Option<usize>,  // Optional weight (default is 1)
}

// Reads and parses config.yml
pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)
        .map_err(|e: io::Error| format!("Failed to read '{}': {}", path, e))?;

    let config: Config =
        serde_yaml::from_str(&contents).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(config)
}
