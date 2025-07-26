mod auth;
mod request_inputs;
mod request_outputs;
mod routes;

use axum::{routing::post, Router};
use std::sync::{Arc, Mutex};    
use std::net::SocketAddr;
use routes::user::signup_route;
use store::store::Store;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    // Create shared store
    let store = Arc::new(Mutex::new(Store::new().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("DB error: {}", e))
    })?));

    // Pass state into Axum app
    let app = Router::new()
        .route("/register", post(signup_route))
        .with_state(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
