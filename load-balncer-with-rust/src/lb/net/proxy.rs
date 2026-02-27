// src/lb/net/proxy.rs
use crate::lb::core::manager::LoadBalancer;
use std::sync::Arc;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

pub async fn start_tcp_listener(addr: String, lb: Arc<LoadBalancer>) -> io::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    println!("PROXY: Listener active on {}", addr);

    loop {
        let (mut client, client_addr) = listener.accept().await?;
        let lb_ref = Arc::clone(&lb);

        tokio::spawn(async move {
            if let Some(server) = lb_ref.select_server() {
                let target_host = server.host.clone();
                server.increment_connections();

                println!(
                    "PROXY: Routing {} -> {} [Load: {}/{}]",
                    client_addr,
                    target_host,
                    server.get_active_connections(),
                    server.max_connections
                );

                if let Ok(mut backend) = TcpStream::connect(&target_host).await {
                    let _ = io::copy_bidirectional(&mut client, &mut backend).await;
                } else {
                    eprintln!("PROXY ERROR: Failed to connect to backend {}", target_host);
                }

                server.decrement_connections();
            } else {
                eprintln!(
                    "PROXY ERROR: No healthy backends for client {}",
                    client_addr
                );
            }
        });
    }
}
