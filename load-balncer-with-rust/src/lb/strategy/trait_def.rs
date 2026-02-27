// src/lb/strategy/trait_def.rs
use crate::lb::core::server::Server;
use std::sync::Arc;

// Strategy trait defines how a server is selected.
pub trait Strategy: Send + Sync {
    fn select<'a>(&self, servers: &'a [Arc<Server>]) -> Option<&'a Arc<Server>>;
}
