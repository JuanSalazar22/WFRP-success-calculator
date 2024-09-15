use axum::{body::Bytes, http::StatusCode};

pub async fn create() -> String {
    "Creating character".to_string()
}

pub async fn update() -> String {
    "Updating Character!".to_string()
}

pub async fn get() -> String {
    "Getting character".to_string()
}

pub async fn get_all() -> String {
    "Getting all characters".to_string()
}

pub async fn delete() -> String {
    "Removing character".to_string()
}