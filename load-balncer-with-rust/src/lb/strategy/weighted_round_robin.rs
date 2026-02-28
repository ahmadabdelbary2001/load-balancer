// src/lb/strategy/weighted_round_robin.rs
use crate::lb::core::server::Server;
use crate::lb::strategy::trait_def::Strategy;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Weighted Round Robin strategy.
/// Selects servers based on their weight (S3 (w:2) gets twice as many requests as S1 (w:1)).
pub struct WeightedRoundRobin {
    counter: AtomicUsize,
}

impl WeightedRoundRobin {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

impl Strategy for WeightedRoundRobin {
    fn select<'a>(&self, servers: &'a [Arc<Server>]) -> Option<&'a Arc<Server>> {
        let healthy: Vec<&Arc<Server>> = servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy.is_empty() {
            println!("STRATEGY (WeightedRR): [!!!] NO HEALTHY SERVERS!");
            return None;
        }

        // Calculate total weight of healthy servers
        let total_weight: usize = healthy.iter().map(|s| s.weight).sum();
        if total_weight == 0 {
            return None;
        }

        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        let mut target = count % total_weight;

        for server in healthy {
            if target < server.weight {
                println!(
                    "STRATEGY (WeightedRR): Attempt #{}. Pool: {:?}. Selected: {} (Weight: {})",
                    count + 1,
                    servers
                        .iter()
                        .filter(|s| s.is_healthy())
                        .map(|s| format!("{}(w:{})", s.host, s.weight))
                        .collect::<Vec<_>>(),
                    server.host,
                    server.weight
                );
                return Some(server);
            }
            target -= server.weight;
        }

        None
    }
}
