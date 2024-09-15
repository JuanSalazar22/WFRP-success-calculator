use crate::handlers;

pub async fn hello_world() -> String {
    "Hello wolrd!".to_string()
}

pub async fn heartbeat() -> String {
    "Running".to_string()
}