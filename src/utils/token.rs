use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{config::env::Env, dtos::user_dto::UserResponse, utils::response::AppError};

#[derive(Serialize, Deserialize)]
pub struct Tokens {
    pub sub: String,
    pub user: UserResponse,
    pub exp: usize,
}

impl Tokens {
    pub fn generate_jwt(user: &UserResponse) -> Result<String, AppError> {
        // 24 hours from now, in seconds
        let expiration = (chrono::Utc::now().timestamp() + 60 * 60 * 24) as usize;

        let claims = Tokens {
            sub: user._id.clone(),
            user: user.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Env::from_env().jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(e.to_string()))
    }

    pub fn verify_token(token: &str) -> Result<Tokens, AppError> {
        // Validation::default() checks the signature AND that exp has not passed
        decode::<Tokens>(
            token,
            &DecodingKey::from_secret(Env::from_env().jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| {
            // Print the real reason on the server, send a plain message to the client
            println!("JWT error: {:?}", e);
            AppError::Unauthorized("Invalid or expired token".to_string())
        })
    }
}
