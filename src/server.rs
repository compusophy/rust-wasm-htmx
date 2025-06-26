use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::fs;

// Message types for WebSocket communication
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "calculation")]
    Calculation {
        operation: String,
        result: f64,
        timestamp: String,
    },
    #[serde(rename = "chat")]
    Chat {
        user: String,
        message: String,
        timestamp: String,
    },
    #[serde(rename = "system")]
    System {
        message: String,
        timestamp: String,
    },
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<WsMessage>>>>;

fn get_timestamp() -> String {
    time::OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "unknown".to_string())
}

#[tokio::main]
async fn main() {
    println!("🦀 Starting Pure Rust Server (HTTP + WebSocket on same port)...");
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    // WebSocket state
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (broadcast_tx, _) = broadcast::channel::<WsMessage>(100);
    
    // Send startup message
    let startup_msg = WsMessage::System {
        message: "🦀 Pure Rust server started!".to_string(),
        timestamp: get_timestamp(),
    };
    let _ = broadcast_tx.send(startup_msg);
    
    println!("🚀 Server running at http://0.0.0.0:{}", port);
    println!("📁 Serving static files + WebSocket upgrades on SAME port");
    println!("🔌 WebSocket available at ws://your-domain/ws");
    println!("🦀 WASM files available at /pkg/");
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind server");
    
    while let Ok((stream, addr)) = listener.accept().await {
        let clients_clone = clients.clone();
        let broadcast_tx_clone = broadcast_tx.clone();
        
        tokio::spawn(async move {
            handle_connection(stream, addr.to_string(), clients_clone, broadcast_tx_clone).await;
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    client_addr: String,
    clients: Clients,
    broadcast_tx: broadcast::Sender<WsMessage>,
) {
    // Read the first line to see if it's an HTTP request
    let mut buf = [0; 1024];
    let n = match stream.peek(&mut buf).await {
        Ok(n) => n,
        Err(_) => return,
    };
    
    let request_str = String::from_utf8_lossy(&buf[..n]);
    
    // Check if it's a WebSocket upgrade request
    if request_str.contains("Upgrade: websocket") || request_str.contains("/ws") {
        println!("🔌 WebSocket upgrade request from {}", client_addr);
        handle_websocket_upgrade(stream, client_addr, clients, broadcast_tx).await;
    } else {
        println!("📨 HTTP request from {}", client_addr);
        handle_http(stream, &request_str).await;
    }
}

async fn handle_websocket_upgrade(
    stream: tokio::net::TcpStream,
    client_id: String,
    clients: Clients,
    broadcast_tx: broadcast::Sender<WsMessage>,
) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("❌ WebSocket upgrade failed for {}: {}", client_id, e);
            return;
        }
    };
    
    println!("🤝 WebSocket connected: {}", client_id);
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    // Add client
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id.clone(), broadcast_tx.clone());
    }
    
    // Welcome message
    let welcome_msg = WsMessage::System {
        message: format!("🦀 Welcome {}!", client_id),
        timestamp: get_timestamp(),
    };
    
    if let Ok(welcome_json) = serde_json::to_string(&welcome_msg) {
        let _ = ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(welcome_json)).await;
    }
    
    // Handle incoming messages
    let broadcast_tx_clone = broadcast_tx.clone();
    let client_id_clone = client_id.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            if let Ok(tokio_tungstenite::tungstenite::Message::Text(text)) = result {
                println!("📨 WS from {}: {}", client_id_clone, text);
                
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(parsed_msg) => {
                        let _ = broadcast_tx_clone.send(parsed_msg);
                    }
                    Err(_) => {
                        let chat_msg = WsMessage::Chat {
                            user: client_id_clone.clone(),
                            message: text,
                            timestamp: get_timestamp(),
                        };
                        let _ = broadcast_tx_clone.send(chat_msg);
                    }
                }
            }
        }
    });
    
    // Handle outgoing broadcasts
    let client_id_clone = client_id.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Ok(broadcast_msg) = broadcast_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&broadcast_msg) {
                if ws_sender
                    .send(tokio_tungstenite::tungstenite::Message::Text(json))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    });
    
    tokio::select! {
        _ = incoming_task => {},
        _ = outgoing_task => {},
    }
    
    // Cleanup
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.remove(&client_id);
    }
    
    println!("🔌 WebSocket closed: {}", client_id);
}

async fn handle_http(stream: tokio::net::TcpStream, request_str: &str) {
    // Parse the request line
    let lines: Vec<&str> = request_str.lines().collect();
    if lines.is_empty() {
        return;
    }
    
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }
    
    let path = parts[1];
    println!("📨 HTTP {} {}", parts[0], path);
    
    let (content, content_type) = match path {
        "/" => (read_file("index.html"), "text/html"),
        "/play" => (read_file("play.html"), "text/html"),
        "/health" => {
            let health_info = format!(
                "{{\"status\":\"ok\",\"server\":\"rust\",\"wasm_files\":[{}]}}",
                if std::path::Path::new("pkg").exists() {
                    let entries: Vec<String> = std::fs::read_dir("pkg")
                        .map(|entries| entries
                            .filter_map(|e| e.ok())
                            .map(|e| format!("\"{}\"", e.file_name().to_string_lossy()))
                            .collect())
                        .unwrap_or_default();
                    entries.join(",")
                } else {
                    "\"pkg directory not found\"".to_string()
                }
            );
            (Some(health_info.into_bytes()), "application/json")
        },
        path if path.starts_with("/pkg/") => {
            let file_path = &path[1..]; // Remove leading /
            let content_type = if path.ends_with(".js") {
                "application/javascript"
            } else if path.ends_with(".wasm") {
                "application/wasm"
            } else {
                "application/octet-stream"
            };
            (read_file(file_path), content_type)
        },
        path if path.starts_with("/public/") => {
            let file_path = &path[1..]; // Remove leading /
            (read_file(file_path), "application/json")
        },
        _ => {
            // Try to serve the file directly
            let file_path = &path[1..]; // Remove leading /
            (read_file(file_path), "text/plain")
        }
    };
    
    let response = match content {
        Some(file_content) => {
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                file_content.len(),
                String::from_utf8_lossy(&file_content)
            )
        }
        None => {
            "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\n404 Error".to_string()
        }
    };
    
    use tokio::io::AsyncWriteExt;
    let mut stream = stream;
    let _ = stream.write_all(response.as_bytes()).await;
}

fn read_file(file_path: &str) -> Option<Vec<u8>> {
    match fs::read(file_path) {
        Ok(content) => {
            println!("✅ Served: {}", file_path);
            Some(content)
        }
        Err(_) => {
            println!("❌ Not found: {}", file_path);
            None
        }
    }
} 