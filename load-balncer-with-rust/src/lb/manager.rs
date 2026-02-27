// load-balncer-with-rust/src/lb/manager.rs

use crate::lb::server::Server;
use crate::lb::strategy::Strategy;
use std::sync::Arc;

// LoadBalancer manages the pool of backend servers and selection logic.
// It uses a strategy to pick the best server for each request.
pub struct LoadBalancer {
    servers: Vec<Arc<Server>>,
    strategy: Box<dyn Strategy>,
}

impl LoadBalancer {
    pub fn new(servers: Vec<Arc<Server>>, strategy: Box<dyn Strategy>) -> Self {
        Self { servers, strategy }
    }

    // Selects a server based on the encapsulated selection strategy.
    pub fn select_server(&self) -> Option<Arc<Server>> {
        self.strategy.select(&self.servers).cloned()
    }

    // Returns a reference to the internal server pool.
    pub fn get_servers(&self) -> &[Arc<Server>] {
        &self.servers
    }
}
