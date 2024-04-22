pub mod handlers;

use crate::handlers::{greet, health_check};

use actix_web::{web::get, App, HttpServer};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(greet))
            .route("/{name}", get().to(greet))
            .route("/{health_check}", get().to(health_check(request)))
    })
    .bind("localhost:8000")?
    .run()
    .await?;

    return Ok(());
}
