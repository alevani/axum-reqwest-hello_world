use axum_macros::FromRef;


#[derive(Clone, Debug)]
pub struct Envs {}

#[derive(Clone, FromRef)]
pub struct State {
    http_client: reqwest::Client,
    envs: Envs,
}

/// Create a new shared State
///
/// Fetches all the environment varaibles from the environment
/// and connects to the postgres database
///
/// Panics if not possible to connect to postgres
/// Panics if it does not find an environment variable
pub(crate) async fn new() -> State {
    // Retrive all necessary environment variables
    let envs = Envs {};

    let reqwest_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .connection_verbose(true)
        .build()
        .expect("Could not create reqwest client");

    State {
        envs,
        http_client: reqwest_client,
    }
}
