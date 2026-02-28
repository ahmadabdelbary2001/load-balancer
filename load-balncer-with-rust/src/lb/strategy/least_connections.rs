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
        let healthy: Vec<&Arc<Server>> = servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy.is_empty() {
            println!("STRATEGY (LeastConnections): No healthy servers available.");
            return None;
        }

        let selected = healthy
            .iter()
            .min_by_key(|s| s.get_active_connections())
            .cloned();

        if let Some(s) = selected {
            println!(
                "STRATEGY (LeastConnections): Selected {} (Active Conns: {})",
                s.host,
                s.get_active_connections()
            );
        }

        selected
    }
}
