use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

// Shared state for connected clients
type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<WsMessage>>>>;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Rust WebSocket Server...");
    
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (broadcast_tx, _) = broadcast::channel::<WsMessage>(100);
    
    // Create a listener on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to port 8080");
    
    println!("🔌 WebSocket server listening on ws://127.0.0.1:8080");
    println!("📡 Broadcasting system messages to all clients");
    
    // Send a system startup message
    let startup_msg = WsMessage::System {
        message: "WebSocket server started! 🦀".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    let _ = broadcast_tx.send(startup_msg);
    
    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        println!("🤝 New connection from: {}", addr);
        
        let clients_clone = clients.clone();
        let broadcast_tx_clone = broadcast_tx.clone();
        
        tokio::spawn(async move {
            handle_connection(stream, addr.to_string(), clients_clone, broadcast_tx_clone).await;
        });
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    client_id: String,
    clients: Clients,
    broadcast_tx: broadcast::Sender<WsMessage>,
) {
    // Upgrade the connection to WebSocket
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("❌ WebSocket upgrade failed for {}: {}", client_id, e);
            return;
        }
    };
    
    println!("✅ WebSocket connection established for {}", client_id);
    
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    // Add client to the clients map
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id.clone(), broadcast_tx.clone());
    }
    
    // Send welcome message to the new client
    let welcome_msg = WsMessage::System {
        message: format!("Welcome! You are connected as {}", client_id),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    if let Ok(welcome_json) = serde_json::to_string(&welcome_msg) {
        let _ = ws_sender.send(Message::Text(welcome_json)).await;
    }
    
    // Handle incoming messages from this client
    let broadcast_tx_clone = broadcast_tx.clone();
    let client_id_clone = client_id.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("📨 Received from {}: {}", client_id_clone, text);
                    
                    // Try to parse as our message format
                    match serde_json::from_str::<WsMessage>(&text) {
                        Ok(parsed_msg) => {
                            // Broadcast the message to all clients
                            let _ = broadcast_tx_clone.send(parsed_msg);
                        }
                        Err(_) => {
                            // If it's not our format, treat it as a simple chat message
                            let chat_msg = WsMessage::Chat {
                                user: client_id_clone.clone(),
                                message: text,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            };
                            let _ = broadcast_tx_clone.send(chat_msg);
                        }
                    }
                }
                Ok(Message::Ping(_data)) => {
                    println!("🏓 Ping from {}", client_id_clone);
                    // Note: tokio-tungstenite handles pong automatically
                }
                Ok(Message::Close(_)) => {
                    println!("👋 Client {} disconnected", client_id_clone);
                    break;
                }
                Err(e) => {
                    println!("❌ Error receiving message from {}: {}", client_id_clone, e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Handle outgoing broadcasts to this client
    let client_id_clone = client_id.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Ok(broadcast_msg) = broadcast_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&broadcast_msg) {
                if ws_sender.send(Message::Text(json)).await.is_err() {
                    println!("❌ Failed to send message to {}", client_id_clone);
                    break;
                }
            }
        }
    });
    
    // Wait for either task to complete (client disconnects or error)
    tokio::select! {
        _ = incoming_task => {},
        _ = outgoing_task => {},
    }
    
    // Remove client from the clients map
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.remove(&client_id);
    }
    
    println!("🔌 Connection closed for {}", client_id);
} 