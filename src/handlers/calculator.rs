use axum::{body::Bytes, http::StatusCode};

pub async fn throw() -> String {
    "Throwing d100 and calculating".to_string()
}

pub async fn calculate_throw() -> String {
    "Calculating throwing result".to_string()
}