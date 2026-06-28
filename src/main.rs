use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;



#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}


#[derive(Debug, Deserialize)]
struct UserPayload {
    name: String,
    email: String,
}


type SharedState = Arc<Mutex<HashMap<String, User>>>;



#[tokio::main]
async fn main() {
    
    let state: SharedState = Arc::new(Mutex::new(HashMap::new()));

    
    let app = Router::new()
        .route("/users", get(get_all_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user_by_id)
                .put(update_user)
                .delete(delete_user),
        )
        .with_state(state);

    println!("Success!! Server is running at http://localhost:3000");
    println!("   Available endpoints:");
    println!("   GET    /users");
    println!("   GET    /users/:id");
    println!("   POST   /users");
    println!("   PUT    /users/:id");
    println!("   DELETE /users/:id");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



// GET — Return all users
async fn get_all_users(State(state): State<SharedState>) -> Json<Vec<User>> {
    let users = state.lock().unwrap();
    let all_users: Vec<User> = users.values().cloned().collect();
    Json(all_users)
}

// GET  — Return a single user by ID
async fn get_user_by_id(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let users = state.lock().unwrap();
    match users.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("User with id '{}' not found", id),
        )),
    }
}

// POST-Create a new user
async fn create_user(
    State(state): State<SharedState>,
    Json(payload): Json<UserPayload>,
) -> (StatusCode, Json<User>) {
    let new_user = User {
        id: Uuid::new_v4().to_string(), // Generate a unique ID
        name: payload.name,
        email: payload.email,
    };

    let mut users = state.lock().unwrap();
    users.insert(new_user.id.clone(), new_user.clone());

    (StatusCode::CREATED, Json(new_user))
}

// PUT — Update an existing user
async fn update_user(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(payload): Json<UserPayload>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut users = state.lock().unwrap();
    match users.get_mut(&id) {
        Some(user) => {
            user.name = payload.name;
            user.email = payload.email;
            Ok(Json(user.clone()))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            format!("User with id '{}' not found", id),
        )),
    }
}

// DELETE — Delete a user
async fn delete_user(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> (StatusCode, String) {
    let mut users = state.lock().unwrap();
    match users.remove(&id) {
        Some(_) => (StatusCode::OK, format!("User '{}' deleted successfully", id)),
        None => (
            StatusCode::NOT_FOUND,
            format!("User with id '{}' not found", id),
        ),
    }
}
