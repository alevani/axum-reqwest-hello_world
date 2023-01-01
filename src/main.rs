use axum::{
    routing::get,
    Router, extract::State,
};
use axum_macros::FromRef;

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Envs {}

#[derive(Clone, FromRef)]
struct AppState {
    http_client: reqwest::Client,
    envs: Envs,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    // Retrive all necessary environment variables
    let envs = Envs {};

    let reqwest_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .connection_verbose(true)
        .build()
        .expect("Could not create reqwest client");

    let states = AppState {
      envs,
      http_client: reqwest_client,  
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(ping))
        .with_state(states);
        
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
