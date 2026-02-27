// src/lb/mod.rs
pub mod core;
pub mod net;
pub mod strategy;

// Re-export common types for a clean API (SOLID)
pub use core::{LoadBalancer, Server};
pub use net::{start_health_checks, start_tcp_listener};
pub use strategy::{RoundRobin, Strategy};
