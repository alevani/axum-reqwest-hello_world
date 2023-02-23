use api::example;
use axum::{routing::get, Router};
use std::net::SocketAddr;

pub mod api;
pub mod errors;
pub mod states;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // build our application with a route
    let app = Router::new()
        .route("/", get(ping))
        .route("/example", get(example))
        .with_state(states::new().await);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    "I am alive!"
}
