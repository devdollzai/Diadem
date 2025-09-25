// @AXIOMHIVE @DEVDOLLZAI :: Generated Sovereign Application

use axum::{routing::get, Router, response::Json};
use serde::Serialize;

#[derive(Serialize)]
struct SignalResponse {
    status: String,
    message: String,
}

async fn handle_signal() -> Json<SignalResponse> {
    println!("Received a signal...");
    Json(SignalResponse {
        status: "OK".to_string(),
        message: "Signal captured. Awaiting processing.".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/signal", get(handle_signal));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Data Moat Service listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}