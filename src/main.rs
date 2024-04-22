pub mod handlers;

use actix_web::{web::get, App, HttpServer};

use handlers::root;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(root))
    })
    .bind("localhost:8000")?
    .run()
    .await?;

    return Ok(());
}