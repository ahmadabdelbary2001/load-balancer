// src/lb/strategy/round_robin.rs
use crate::lb::core::server::Server;
use crate::lb::strategy::trait_def::Strategy;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Simple Round Robin implementation
pub struct RoundRobin {
    counter: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

impl Strategy for RoundRobin {
    fn select<'a>(&self, servers: &'a [Arc<Server>]) -> Option<&'a Arc<Server>> {
        let healthy_servers: Vec<&Arc<Server>> =
            servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy_servers.is_empty() {
            return None;
        }

        let index = self.counter.fetch_add(1, Ordering::Relaxed);
        Some(healthy_servers[index % healthy_servers.len()])
    }
}
