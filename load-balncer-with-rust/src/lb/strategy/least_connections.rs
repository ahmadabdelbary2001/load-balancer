// src/lb/strategy/least_connections.rs
use crate::lb::core::server::Server;
use crate::lb::strategy::trait_def::Strategy;
use std::sync::Arc;

// Least Connections implementation
pub struct LeastConnections;

impl LeastConnections {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for LeastConnections {
    fn select<'a>(&self, servers: &'a [Arc<Server>]) -> Option<&'a Arc<Server>> {
        servers
            .iter()
            .filter(|s| s.is_healthy())
            .min_by_key(|s| s.get_active_connections())
    }
}
