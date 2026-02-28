// src/lb/strategy/round_robin.rs
use crate::lb::core::server::Server;
use crate::lb::strategy::trait_def::Strategy;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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
        let healthy: Vec<&Arc<Server>> = servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy.is_empty() {
            println!("STRATEGY (RoundRobin): [!!!] NO HEALTHY SERVERS in the pool!");
            return None;
        }

        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        let idx = count % healthy.len();
        let selected = healthy[idx];

        println!(
            "STRATEGY (RoundRobin): Attempt #{}. Pool: {:?}. Selected: {} (at index {})",
            count + 1,
            healthy.iter().map(|s| &s.host).collect::<Vec<_>>(),
            selected.host,
            idx
        );

        Some(selected)
    }
}
