use std::future::{Ready, ready};

use actix_web::{
    Error, FromRequest, HttpMessage, HttpRequest,
    body::MessageBody,
    dev::{Payload, ServiceRequest, ServiceResponse},
    middleware::Next,
};

use crate::{
    dtos::user_dto::UserResponse,
    utils::{response::AppError, token::Tokens},
};

// THE GUARD: runs before every route in a wrapped scope
pub async fn auth_guard(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Read the Authorization header as text
    let header = req
        .headers()
        .get("Authorization")
        .ok_or(AppError::Unauthorized("Missing token".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    // Remove "Bearer " from the front
    let token = header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid token".to_string()))?;

    // Check the token and pull out the claims
    let claims = Tokens::verify_token(token)?;

    // Drop the user into the request's bag for the handler to pick up
    req.extensions_mut().insert(claims.user);

    // Token is good, carry on to the handler
    next.call(req).await
}

pub struct AuthUser {
    pub user: UserResponse,
}

impl FromRequest for AuthUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let user = req.extensions().get::<UserResponse>().cloned();

        ready(
            user.map(|user| AuthUser { user })
                .ok_or(AppError::Unauthorized("Not authenticated".to_string())),
        )
    }
}
