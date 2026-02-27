// load-balncer-with-rust/src/main.rs

mod config;
mod lb;

use lb::{LoadBalancer, RoundRobin, Server};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Stage 1: Load configuration
    let cfg = config::load_config("config.yml").expect("Failed to load config.yml");

    println!("--- Multi-Protocol Load Balancer Started ---");
    println!("Health Check Interval: {}", cfg.health_check_interval);

    // Stage 2: Initialize server pool
    let mut servers = Vec::new();
    for s_cfg in &cfg.servers {
        println!(
            "CONFIG: Backend Server {} (Mode: {}, Max Conns: {})",
            s_cfg.host, s_cfg.mode, s_cfg.max_connections
        );
        let server = Arc::new(Server::new(s_cfg.host.clone(), s_cfg.max_connections));
        servers.push(server);
    }

    // Wrap servers for health check sharing
    let shared_servers = servers.clone();

    // Stage 3: Start Health Checks (Background)
    let health_interval = Duration::from_secs(5);
    tokio::spawn(async move {
        lb::start_health_checks(shared_servers, health_interval).await;
    });

    // Stage 4: Start Listeners based on config
    for listener_cfg in cfg.listeners {
        let pool = servers.clone();
        let addr = listener_cfg.listen_addr.clone();
        let algo = listener_cfg.algorithm.clone();

        // SOLID: Choose strategy based on config
        let strategy: Box<dyn lb::Strategy> = match algo.as_str() {
            "round_robin" => Box::new(RoundRobin::new()),
            _ => {
                println!(
                    "WARNING: Algorithm '{}' not implemented, falling back to RoundRobin",
                    algo
                );
                Box::new(RoundRobin::new())
            }
        };

        let lb = Arc::new(LoadBalancer::new(pool, strategy));
        println!(
            "STARTUP: Listener on {} (Mode: {}, Algo: {}) - Pool size: {}",
            addr,
            listener_cfg.mode,
            algo,
            lb.get_servers().len()
        );

        tokio::spawn(async move {
            let addr_str = addr.clone();
            match listener_cfg.mode.as_str() {
                "tcp" | "http" => {
                    if let Err(e) = lb::start_tcp_listener(addr, lb).await {
                        eprintln!("LISTENER ERROR {}: {}", addr_str, e);
                    }
                }
                _ => eprintln!("UNSUPPORTED MODE: {}", listener_cfg.mode),
            }
        });
    }

    // Keep the main task alive
    loop {
        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
