// src/lb/net/proxy.rs
use crate::lb::core::manager::LoadBalancer;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
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
            let mut attempts = 0;
            let max_retries = 3;
            let mut connection_successful = false;

            while attempts < max_retries && !connection_successful {
                // TCP Proxy uses default selection (no path available)
                if let Some(server) = lb_ref.select_server_with_path("") {
                    let target_host = server.host.clone();

                    // Capacity Guard: Check if server is overloaded
                    if server.get_active_connections() >= server.max_connections {
                        println!(
                            "L4 PROXY [TCP]: Server {} BUSY ({}/{}), retrying another...",
                            target_host,
                            server.get_active_connections(),
                            server.max_connections
                        );
                        continue;
                    }

                    server.increment_connections();
                    attempts += 1;

                    println!(
                        "L4 PROXY [TCP]: Attempt {}/{} | Client {} -> {}",
                        attempts, max_retries, client_addr, target_host
                    );

                    match TcpStream::connect(&target_host).await {
                        Ok(mut backend) => {
                            println!("L4 PROXY [TCP]: SUCCESS - Connected to {}", target_host);
                            connection_successful = true;
                            let _ = io::copy_bidirectional(&mut client, &mut backend).await;
                            println!("L4 PROXY [TCP]: CLOSED connection to {}", target_host);
                        }
                        Err(e) => {
                            eprintln!(
                                "L4 PROXY ERROR [TCP]: Failed to connect to {} ({}). Marking UNHEALTHY.",
                                target_host, e
                            );
                            server.set_healthy(false);
                        }
                    }
                    server.decrement_connections();
                } else {
                    println!("L4 PROXY [TCP]: No available servers for selection.");
                    break;
                }
            }

            if !connection_successful {
                let error_response =
                    "HTTP/1.1 503 Service Unavailable\r\nConnection: close\r\n\r\n";
                let _ = io::AsyncWriteExt::write_all(&mut client, error_response.as_bytes()).await;
            }
        });
    }
}

/// Layer 7 HTTP Proxy Listener (Request-based)
pub async fn start_http_listener(addr: String, lb: Arc<LoadBalancer>) -> io::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    println!("PROXY (HTTP): Listener active on {}", addr);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let lb_ref = Arc::clone(&lb);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                handle_http_request(Arc::clone(&lb_ref), req, client_addr.to_string())
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!(
                    "HTTP PROXY ERROR: Error serving connection to {}: {}",
                    client_addr, err
                );
            }
        });
    }
}

async fn handle_http_request(
    lb: Arc<LoadBalancer>,
    req: Request<hyper::body::Incoming>,
    client_ip: String,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut attempts = 0;
    let max_retries = 3;

    // Buffer body for retries
    let (parts, incoming_body) = req.into_parts();
    let buffered_body = match incoming_body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => Bytes::new(),
    };

    while attempts < max_retries {
        let path = parts.uri.path().to_string();
        if let Some(server) = lb.select_server_with_path(&path) {
            let target_host = server.host.clone();

            // Capacity Guard: Check if server is overloaded
            if server.get_active_connections() >= server.max_connections {
                println!(
                    "L7 PROXY [HTTP]: Server {} BUSY ({}/{}), retrying another...",
                    target_host,
                    server.get_active_connections(),
                    server.max_connections
                );
                continue;
            }

            server.increment_connections();
            attempts += 1;

            println!(
                "L7 PROXY: Request {} -> {} [Path: {}] [Attempt: {}]",
                client_ip, target_host, path, attempts
            );

            match TcpStream::connect(&target_host).await {
                Ok(stream) => {
                    let io = TokioIo::new(stream);
                    match hyper::client::conn::http1::handshake(io).await {
                        Ok((mut sender, conn)) => {
                            tokio::spawn(async move {
                                if let Err(e) = conn.await {
                                    eprintln!("L7 BACKEND Connection Error: {}", e);
                                }
                            });

                            let mut builder = Request::builder()
                                .method(parts.method.clone())
                                .uri(parts.uri.clone())
                                .version(parts.version);
                            for (key, value) in parts.headers.iter() {
                                builder = builder.header(key, value);
                            }

                            let client_req =
                                builder.body(Full::new(buffered_body.clone())).unwrap();

                            match sender.send_request(client_req).await {
                                Ok(resp) => {
                                    server.decrement_connections();
                                    let (resp_parts, resp_body) = resp.into_parts();
                                    let body_bytes = match resp_body.collect().await {
                                        Ok(collected) => collected.to_bytes(),
                                        Err(_) => Bytes::new(),
                                    };
                                    let mut res_builder = Response::builder()
                                        .status(resp_parts.status)
                                        .version(resp_parts.version)
                                        // Prevents browser from caching the response so rotation is visible
                                        .header(
                                            "Cache-Control",
                                            "no-cache, no-store, must-revalidate",
                                        )
                                        .header("Pragma", "no-cache")
                                        // Diagnostic headers to solve the user's "staying on same server" mystery
                                        .header("X-Backend-Server", &target_host)
                                        .header(
                                            "X-Selection-Log",
                                            format!("Attempt #{}", attempts),
                                        );

                                    for (key, value) in resp_parts.headers.iter() {
                                        res_builder = res_builder.header(key, value);
                                    }
                                    return Ok(res_builder.body(Full::new(body_bytes)).unwrap());
                                }
                                Err(e) => {
                                    eprintln!("L7 BACKEND Send Error: {}. Retrying...", e);
                                    server.set_healthy(false);
                                    server.decrement_connections();
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("L7 BACKEND Handshake Error: {}. Retrying...", e);
                            server.set_healthy(false);
                            server.decrement_connections();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("BACKEND Connect Error: {}. Retrying...", e);
                    server.set_healthy(false);
                    server.decrement_connections();
                }
            }
        } else {
            break;
        }
    }

    let error_html = r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>Service Unavailable</title>
<style>
body { background: linear-gradient(135deg, #2193b0, #6dd5ed); font-family: sans-serif; margin: 0; display: flex; align-items: center; justify-content: center; min-height: 100vh; }
.container { background: #ffffff; padding: 50px 30px; border-radius: 12px; box-shadow: 0 8px 16px rgba(0,0,0,0.15); text-align: center; max-width: 500px; width: 90%; }
h1 { color: #e74c3c; font-size: 32px; }
p { color: #555; font-size: 18px; }
</style></head>
<body><div class="container"><h1>503 - Service Unavailable</h1><p>Sorry, all backend servers are offline.</p></div></body></html>"#;

    Ok(Response::builder()
        .status(503)
        .header("Content-Type", "text/html")
        .body(Full::new(Bytes::from(error_html)))
        .unwrap())
}
