// load-balncer-with-rust/src/lb/strategy.rs

use super::server::Server;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RoundRobin {
    counter: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    pub fn select<'a>(&self, servers: &'a [Server]) -> Option<&'a Server> {
        let healthy: Vec<&Server> = servers.iter().filter(|s| s.is_healthy()).collect();

        if healthy.is_empty() {
            return None;
        }

        let idx = self.counter.fetch_add(1, Ordering::Relaxed);
        Some(healthy[idx % healthy.len()])
    }
}
