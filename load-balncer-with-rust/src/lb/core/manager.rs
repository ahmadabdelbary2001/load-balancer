// load-balncer-with-rust/src/lb/manager.rs
use crate::lb::core::server::Server;
use crate::lb::strategy::Strategy;
use std::sync::Arc;

use crate::config::AclConfig;

// Manages server pool and selection logic
pub struct LoadBalancer {
    servers: Vec<Arc<Server>>,
    strategy: Box<dyn Strategy>,
    acls: Vec<AclConfig>, // Path-based routing rules
}

impl LoadBalancer {
    pub fn new(
        servers: Vec<Arc<Server>>,
        strategy: Box<dyn Strategy>,
        acls: Vec<AclConfig>,
    ) -> Self {
        Self {
            servers,
            strategy,
            acls,
        }
    }

    // Selects a server based on request path (ACL)
    pub fn select_server_with_path(&self, path: &str) -> Option<Arc<Server>> {
        // 1. Check if any ACL matches the path ending
        for acl in &self.acls {
            if path.ends_with(&acl.pattern) {
                // Filter pool to only include ACL targets
                let acl_pool: Vec<Arc<Server>> = self
                    .servers
                    .iter()
                    .filter(|s| acl.target_hosts.contains(&s.host))
                    .cloned()
                    .collect();

                if !acl_pool.is_empty() {
                    println!(
                        "ACL: Path '{}' matched rule '{}'. Filtering pool to: {:?}",
                        path, acl.name, acl.target_hosts
                    );
                    return self.strategy.select(&acl_pool).cloned();
                }
            }
        }

        // 2. Default: use full pool
        self.strategy.select(&self.servers).cloned()
    }

    // Returns a reference to the internal server pool.
    pub fn get_servers(&self) -> &[Arc<Server>] {
        &self.servers
    }
}
