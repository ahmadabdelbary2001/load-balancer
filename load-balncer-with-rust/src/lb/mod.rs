// src/lb/mod.rs
pub mod core;
pub mod strategy;

// Re-export common types for a clean API (SOLID)
pub use core::{LoadBalancer, Server};
pub use strategy::{RoundRobin, Strategy};
