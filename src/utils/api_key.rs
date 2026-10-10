use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};

use crate::{config::env::Env, utils::response::AppError};

pub async fn api_key_guard(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Paths that need NO api key. Add your docs path here.
    let open_paths = ["/docs", "/health"];

    let path = req.path();
    let is_open = open_paths.iter().any(|p| path.starts_with(p));

    if is_open {
        // Skip the check, carry on to the next step
        return next.call(req).await;
    }

    // Read the x-api-key header
    let provided = req
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    let expected = Env::from_env().api_key;

    match provided {
        Some(key) if key == expected => next.call(req).await,
        _ => Err(AppError::Unauthorized("Invalid or missing API key".to_string()).into()),
    }
}
