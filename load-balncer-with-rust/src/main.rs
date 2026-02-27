// load-balncer-with-rust/src/main.rs

mod config;

fn main() {
    let cfg = config::load_config("config.yml").expect("Failed to load config.yml");

    println!("Loaded config:");
    println!("  health_check_interval: {}", cfg.health_check_interval);
    println!("  listeners: {}", cfg.listeners.len());
    println!("  servers: {}", cfg.servers.len());

    for listener in &cfg.listeners {
        println!(
            "  [Listener] {} | mode={} | algorithm={}",
            listener.listen_addr, listener.mode, listener.algorithm
        );
    }

    for server in &cfg.servers {
        println!(
            "  [Server] {} | max_conn={} | mode={}",
            server.host, server.max_connections, server.mode
        );
    }
}
