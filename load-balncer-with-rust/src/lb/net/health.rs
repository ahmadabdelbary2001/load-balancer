// src/lb/net/health.rs
use crate::lb::core::server::Server;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub async fn start_health_checks(servers: Vec<Arc<Server>>, interval: Duration) {
    let mut ticker = time::interval(interval);

    loop {
        ticker.tick().await;
        for server in &servers {
            let host = server.host.clone();
            let server_ref = Arc::clone(server);

            tokio::spawn(async move {
                let was_healthy = server_ref.is_healthy();
                let is_healthy = tokio::net::TcpStream::connect(&host).await.is_ok();

                if is_healthy != was_healthy {
                    if is_healthy {
                        println!("HEALTH CHECK: [UP]   Server {} is now reachable.", host);
                    } else {
                        eprintln!("HEALTH CHECK: [DOWN] Server {} is UNREACHABLE!", host);
                    }
                    server_ref.set_healthy(is_healthy);
                }
            });
        }
    }
}
