use axum::extract::State;

use crate::{errors::AppError, Envs};

pub(crate) async fn example(
    State(envs): State<Envs>,
    State(client): State<reqwest::Client>,
) -> Result<(), AppError> {
    Ok(())
}
