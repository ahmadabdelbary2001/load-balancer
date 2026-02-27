// load-balncer-with-rust/src/lb/strategy.rs

use crate::lb::Server;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Strategy trait defines how a server is selected.
// This allows adding more algorithms later (SOLID principle).
pub trait Strategy: Send + Sync {
    fn select<'a>(&self, servers: &'a [Arc<Server>]) -> Option<&'a Arc<Server>>;
}

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
        // Only pick from healthy servers
        let healthy_servers: Vec<&Arc<Server>> =
            servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy_servers.is_empty() {
            return None;
        }

        let index = self.counter.fetch_add(1, Ordering::Relaxed);
        Some(healthy_servers[index % healthy_servers.len()])
    }
}
