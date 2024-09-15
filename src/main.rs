use axum::{
    routing::{delete, get, post, put},
    Router,
};

mod handlers;
mod response;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", 
            get(handlers::utility::hello_world))
        .route("/health-check", 
            get(handlers::utility::heartbeat))
        .route("/character", 
            get(handlers::characters::get_all).
            post(handlers::characters::create))
        .route("/character/:id", 
            get(handlers::characters::get).
            put(handlers::characters::update).
            delete(handlers::characters::delete))
        .route("/calculate/throw", 
            post(handlers::calculator::throw))
        .route("/calculate", 
            post(handlers::calculator::calculate_throw));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}