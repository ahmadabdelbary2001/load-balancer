// load-balncer-with-rust/src/main.rs

mod config;
mod lb;

use lb::{RoundRobin, Server};

fn main() {
    // Load config from file
    let cfg = config::load_config("config.yml").expect("Failed to load config.yml");

    println!("Loaded config successfully.");
    println!("Health Check Interval: {}", cfg.health_check_interval);

    // Initialize servers from config to fix dead code warnings
    let mut servers = Vec::new();
    for s_cfg in &cfg.servers {
        println!(
            "Found server: {} (mode: {}, max_conn: {})",
            s_cfg.host, s_cfg.mode, s_cfg.max_connections
        );
        servers.push(Server::new(s_cfg.host.clone()));
    }

    let lb = RoundRobin::new();

    // Demonstrate listener config usage
    let algo_name = if let Some(l) = cfg.listeners.first() {
        println!(
            "\nPrimary Listener: {} | Mode: {} | Algorithm: {}",
            l.listen_addr, l.mode, l.algorithm
        );
        l.algorithm.as_str()
    } else {
        "round_robin"
    };

    println!("\n--- Algorithm Demo ({}) ---", algo_name);
    for i in 1..=6 {
        if let Some(s) = lb.select(&servers) {
            println!("Request {}: Route to {}", i, s.host);
        }
    }
}
