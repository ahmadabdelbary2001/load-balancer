mod config;
mod lb;

use lb::{LoadBalancer, RoundRobin, Server};
use std::sync::Arc;

fn main() {
    // Stage 1: Load and display configuration
    let cfg = config::load_config("config.yml").expect("Failed to load config.yml");

    println!("--- Load Balancer Initialized ---");
    println!("Health Check Interval: {}", cfg.health_check_interval);

    // Demonstrate listener usage to fix warnings
    for listener in &cfg.listeners {
        println!(
            "Configuration: Listening on {} (Mode: {}, Algorithm: {})",
            listener.listen_addr, listener.mode, listener.algorithm
        );
    }

    // Stage 2: Initialize server pool
    let mut servers = Vec::new();
    for s_cfg in &cfg.servers {
        // Use all server config fields
        println!(
            "Server added: {} (mode: {}, max_conn: {})",
            s_cfg.host, s_cfg.mode, s_cfg.max_connections
        );
        let server = Arc::new(Server::new(s_cfg.host.clone(), s_cfg.max_connections));
        servers.push(server);
    }

    // Setup the selection strategy (Round Robin)
    let strategy = Box::new(RoundRobin::new());

    // Create the LoadBalancer instance
    let lb = LoadBalancer::new(servers, strategy);

    // Verify server list access
    println!("Total backend servers: {}", lb.get_servers().len());

    println!("\n--- Load Balancer Selection & Connection Demo ---");
    for i in 1..=6 {
        if let Some(s) = lb.select_server() {
            // Demonstrate connection tracking
            s.increment_connections();

            println!(
                "Request {}: Target -> {} | Active Conns: {}/{} | Healthy: {}",
                i,
                s.host,
                s.get_active_connections(),
                s.max_connections,
                s.is_healthy()
            );

            // Simulation: Update health or connections
            if i % 3 == 0 {
                println!("  (Simulating health status change for {})", s.host);
                s.set_healthy(false);
            }

            s.decrement_connections();
        } else {
            println!("Request {}: Error - No healthy servers", i);
        }
    }
}
