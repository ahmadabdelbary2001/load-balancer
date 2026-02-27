// load-balncer-with-rust/src/lb/server.rs

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// Server struct represents a backend server.
// It uses atomic types for thread-safe shared state.
pub struct Server {
    pub host: String,
    pub max_connections: usize,
    active_connections: AtomicUsize,
    healthy: AtomicBool,
}

impl Server {
    pub fn new(host: String, max_connections: usize) -> Self {
        Self {
            host,
            max_connections,
            active_connections: AtomicUsize::new(0),
            healthy: AtomicBool::new(true),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Relaxed)
    }

    pub fn set_healthy(&self, state: bool) {
        self.healthy.store(state, Ordering::Relaxed);
    }

    pub fn get_active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }
}
