mod config;
mod lb;

// Load core parts from lb module
use lb::{LoadBalancer, Server};
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
        let server = Arc::new(Server::new(
            s_cfg.host.clone(),
            s_cfg.max_connections,
            s_cfg.weight.unwrap_or(1),
        ));
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

        // Select the Load Balancing strategy
        let strategy: Box<dyn lb::Strategy> = match algo.as_str() {
            "round_robin" => Box::new(lb::RoundRobin::new()),
            "least_connections" => Box::new(lb::LeastConnections::new()),
            "weighted_round_robin" => Box::new(lb::WeightedRoundRobin::new()),
            _ => {
                println!(
                    "WARNING: Algorithm '{}' not implemented, falling back to RoundRobin",
                    algo
                );
                Box::new(lb::RoundRobin::new())
            }
        };

        let lb = Arc::new(LoadBalancer::new(
            pool,
            strategy,
            listener_cfg.acls.unwrap_or_default(),
        ));
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
                "http" => {
                    if let Err(e) = lb::start_http_listener(addr, lb).await {
                        eprintln!("HTTP LISTENER ERROR {}: {}", addr_str, e);
                    }
                }
                "tcp" => {
                    if let Err(e) = lb::start_tcp_listener(addr, lb).await {
                        eprintln!("TCP LISTENER ERROR {}: {}", addr_str, e);
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
