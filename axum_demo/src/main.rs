use axum::{
    extract::{Path, Query, WebSocketUpgrade, ws::{WebSocket, Message}},
    http::StatusCode,
    response::{Json, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer, services::ServeDir};
use tracing_subscriber;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UserQuery {
    limit: Option<u32>,
    offset: Option<u32>,
}


async fn get_user(Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };
    Ok(Json(user))
}

async fn list_users(Query(params): Query<UserQuery>) -> Json<Vec<User>> {
    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);
    
    let users: Vec<User> = (offset..offset + limit)
        .map(|i| User {
            id: i,
            name: format!("User {}", i),
            email: format!("user{}@example.com", i),
        })
        .collect();
    
    Json(users)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<User>, StatusCode> {
    let user = User {
        id: 1,
        name: payload.name,
        email: payload.email,
    };
    Ok(Json(user))
}

async fn health_check() -> Json<HashMap<&'static str, &'static str>> {
    let mut response = HashMap::new();
    response.insert("status", "healthy");
    response.insert("version", "0.1.0");
    Json(response)
}

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("WebSocket connection established");
    
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("Received: {}", text);
                    
                    let response = if text.starts_with("echo:") {
                        text.replacen("echo:", "Server echoed:", 1)
                    } else if text == "ping" {
                        "pong".to_string()
                    } else if text == "time" {
                        format!("Current time: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
                    } else {
                        format!("Server received: {}", text)
                    };
                    
                    if let Err(e) = socket.send(Message::Text(response)).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed");
                    break;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/api/health", get(health_check))
        .route("/api/users", get(list_users).post(create_user))
        .route("/api/users/:id", get(get_user))
        .nest_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}