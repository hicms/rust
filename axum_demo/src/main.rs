use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
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

async fn hello_world() -> &'static str {
    "Hello, Axum World!"
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}