use std::error::Error;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create router with a single route
    let router = Router::new().route("/", get(|| async { "Hello, world !" }));

    // Run the server
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
