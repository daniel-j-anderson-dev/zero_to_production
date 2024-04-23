pub mod handlers;

use tokio::net::TcpListener;

use axum::{routing::get, Router};

use crate::handlers::{greet, health_check};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server_address = std::env::args().nth(1).unwrap_or_else(|| String::from("localhost:8000"));

    let listener = TcpListener::bind(server_address).await?;

    let router = Router::new()
        .route("/", get(health_check))
        .route("/:name", get(greet));

    axum::serve(listener, router).await?;

    Ok(())
}
