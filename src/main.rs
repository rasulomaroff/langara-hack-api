use std::net::SocketAddr;

use anyhow::Error;
use axum::{routing::get_service, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let router = Router::new()
        .merge(routes::html::routes())
        .merge(routes::token::routes())
        .merge(routes::voice::routes())
        .layer(cors)
        .fallback_service(
            Router::new().nest_service("/", get_service(ServeDir::new("/home/html"))),
        );

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();

    println!("\nlistening on {}", socket_addr);

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
