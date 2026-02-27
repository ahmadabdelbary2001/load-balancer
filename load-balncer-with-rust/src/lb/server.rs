// load-balncer-with-rust/src/lb/server.rs

use std::sync::atomic::{AtomicBool, Ordering};

pub struct Server {
    pub host: String,
    pub healthy: AtomicBool,
}

impl Server {
    pub fn new(host: String) -> Self {
        Self {
            host,
            healthy: AtomicBool::new(true),
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::Relaxed)
    }
}
