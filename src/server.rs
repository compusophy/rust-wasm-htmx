use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use warp::Filter;
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
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<WsMessage>>>>;

#[tokio::main]
async fn main() {
    println!("🦀 Starting Pure Rust Server...");
    
    // Initialize WebSocket state
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let (broadcast_tx, _) = broadcast::channel::<WsMessage>(100);
    
    // Send startup message
    let startup_msg = WsMessage::System {
        message: "🦀 Pure Rust server started!".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let _ = broadcast_tx.send(startup_msg);
    
    // Static file serving
    let static_files = warp::fs::dir(".")
        .with(warp::log("static"));
    
    // Serve play.html at /play route
    let play_route = warp::path("play")
        .and(warp::get())
        .and(warp::fs::file("play.html"));
    
    // WebSocket route
    let clients_filter = warp::any().map(move || clients.clone());
    let broadcast_filter = warp::any().map(move || broadcast_tx.clone());
    
    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(clients_filter)
        .and(broadcast_filter)
        .map(|ws: warp::ws::Ws, clients: Clients, broadcast_tx: broadcast::Sender<WsMessage>| {
            ws.on_upgrade(move |socket| handle_websocket(socket, clients, broadcast_tx))
        });
    
    // CORS for development
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);
    
    // Combine routes
    let routes = websocket
        .or(play_route)
        .or(static_files)
        .with(cors)
        .with(warp::log("requests"));
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    println!("🚀 Server running at http://localhost:{}", port);
    println!("📁 Serving static files");
    println!("🔌 WebSocket available at ws://localhost:{}/ws", port);
    println!("🦀 WASM files available at /pkg/");
    println!("⚡ Pure Rust - No Node.js dependency!");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

async fn handle_websocket(
    websocket: warp::ws::WebSocket,
    clients: Clients,
    broadcast_tx: broadcast::Sender<WsMessage>,
) {
    let client_id = format!("client_{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) / 1_000_000);
    println!("🤝 New WebSocket connection: {}", client_id);
    
    let (mut ws_sender, mut ws_receiver) = websocket.split();
    let mut broadcast_rx = broadcast_tx.subscribe();
    
    // Add client
    {
        let mut clients_lock = clients.lock().await;
        clients_lock.insert(client_id.clone(), broadcast_tx.clone());
    }
    
    // Welcome message
    let welcome_msg = WsMessage::System {
        message: format!("🦀 Welcome {}! Pure Rust server", client_id),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    if let Ok(welcome_json) = serde_json::to_string(&welcome_msg) {
        let _ = ws_sender.send(warp::ws::Message::text(welcome_json)).await;
    }
    
    // Handle incoming messages
    let broadcast_tx_clone = broadcast_tx.clone();
    let client_id_clone = client_id.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_text() {
                        if let Ok(text) = msg.to_str() {
                            println!("📨 Received from {}: {}", client_id_clone, text);
                            
                            match serde_json::from_str::<WsMessage>(text) {
                                Ok(parsed_msg) => {
                                    let _ = broadcast_tx_clone.send(parsed_msg);
                                }
                                Err(_) => {
                                    let chat_msg = WsMessage::Chat {
                                        user: client_id_clone.clone(),
                                        message: text.to_string(),
                                        timestamp: chrono::Utc::now().to_rfc3339(),
                                    };
                                    let _ = broadcast_tx_clone.send(chat_msg);
                                }
                            }
                        }
                    } else if msg.is_close() {
                        println!("👋 Client {} disconnected", client_id_clone);
                        break;
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
    let client_id_clone_outgoing = client_id.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Ok(broadcast_msg) = broadcast_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&broadcast_msg) {
                if ws_sender
                    .send(warp::ws::Message::text(json))
                    .await
                    .is_err()
                {
                    println!("❌ Failed to send message to {}", client_id_clone_outgoing);
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