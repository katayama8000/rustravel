use axum::{
    routing::get,
    Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!!!" }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("Listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}