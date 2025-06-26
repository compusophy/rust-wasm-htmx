use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::fs;
use tokio::sync::{broadcast, Mutex};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tiny_http::{Server, Response, Header};

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
    println!("🦀 Starting Pure Rust Server with WebSockets...");
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    
    // WebSocket state
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (broadcast_tx, _) = broadcast::channel::<WsMessage>(100);
    
    // Send startup message
    let startup_msg = WsMessage::System {
        message: "🦀 Pure Rust server with WebSockets started!".to_string(),
        timestamp: get_timestamp(),
    };
    let _ = broadcast_tx.send(startup_msg);
    
    println!("🚀 Server running at http://localhost:{}", port);
    println!("📁 Serving static files via HTTP");
    println!("🔌 WebSocket available at ws://localhost:{}/ws", port);
    println!("🦀 WASM files available at /pkg/");
    println!("⚡ Pure Rust - Minimal C dependencies!");
    
    // Start HTTP server in a separate thread
    let http_port = port.clone();
    thread::spawn(move || {
        start_http_server(&http_port);
    });
    
    // Start WebSocket server on same port + 1
    let ws_port: u16 = port.parse::<u16>().unwrap_or(3000) + 1;
    start_websocket_server(ws_port, clients, broadcast_tx).await;
}

fn start_http_server(port: &str) {
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    
    for request in server.incoming_requests() {
        thread::spawn(move || {
            handle_http_request(request);
        });
    }
}

fn handle_http_request(request: tiny_http::Request) {
    let url = request.url();
    let method = request.method();
    
    println!("📨 {} {}", method, url);
    
    let response = match url {
        "/" => serve_file("index.html"),
        "/play" => serve_file("play.html"),
        path if path.starts_with("/pkg/") => serve_file(&path[1..]),
        path if path.starts_with("/public/") => serve_file(&path[1..]),
        _ => serve_file(&url[1..])
    };
    
    let _ = request.respond(response);
}

fn serve_file(file_path: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    match fs::read(file_path) {
        Ok(contents) => {
            let content_type = match file_path.split('.').last() {
                Some("html") => "text/html",
                Some("js") => "application/javascript",
                Some("wasm") => "application/wasm",
                Some("json") => "application/json",
                Some("css") => "text/css",
                _ => "text/plain",
            };
            
            let header = Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
                .expect("Invalid header");
            
            Response::from_data(contents).with_header(header)
        }
        Err(_) => {
            if file_path != "index.html" {
                return serve_file("index.html");
            }
            Response::from_string("404 Not Found").with_status_code(404)
        }
    }
}

async fn start_websocket_server(
    port: u16, 
    clients: Clients, 
    broadcast_tx: broadcast::Sender<WsMessage>
) {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind WebSocket server");
    
    println!("🔌 WebSocket server listening on port {}", port);
    
    while let Ok((stream, addr)) = listener.accept().await {
        let clients_clone = clients.clone();
        let broadcast_tx_clone = broadcast_tx.clone();
        
        tokio::spawn(async move {
            handle_websocket_connection(stream, addr.to_string(), clients_clone, broadcast_tx_clone).await;
        });
    }
}

async fn handle_websocket_connection(
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
    
    println!("🤝 New WebSocket connection: {}", client_id);
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    // Add client
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id.clone(), broadcast_tx.clone());
    }
    
    // Welcome message
    let welcome_msg = WsMessage::System {
        message: format!("🦀 Welcome {}! WebSocket connected", client_id),
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
            match result {
                Ok(msg) => {
                    if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                        println!("📨 Received from {}: {}", client_id_clone, text);
                        
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
                Err(e) => {
                    println!("❌ WebSocket error for {}: {}", client_id_clone, e);
                    break;
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
                    println!("❌ Failed to send message to {}", client_id_clone);
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
    
    println!("🔌 Connection closed for {}", client_id);
} 