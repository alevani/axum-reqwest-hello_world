use api::example;
use axum::{routing::get, Router};
use tokio::net::TcpListener;

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
    let tcp_listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    "I am alive!"
}
