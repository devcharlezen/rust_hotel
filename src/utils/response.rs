use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::{Value, json};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Conflict(String),
    NotFound(String),
    Internal(String),
    Unauthorized(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Conflict(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for AppError {
    // Which status code each kind gets
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "status": false,
            "message": self.to_string(),
            "data": null
        }))
    }
}

pub fn success_response(code: Option<StatusCode>, message: &str, data: Value) -> HttpResponse {
    let code = code.unwrap_or(StatusCode::OK);

    HttpResponse::build(code).json(json!({
        "status": true,
        "message": message,
        "data": data
    }))
}
