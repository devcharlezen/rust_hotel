use actix_web::{
    HttpResponse,
    http::StatusCode,
    web::{Data, Json},
};
use validator::Validate;

use crate::{
    config::db::Database,
    dtos::user_dto,
    services::user_service,
    utils::{
        auth::AuthUser,
        response::{AppError, success_response},
    },
};

pub async fn create_user(
    payload: Json<user_dto::CreateUserDto>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    // Run the validation rules on the DTO
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Take the plain DTO out of the Json wrapper
    let payload = payload.into_inner();
    let user_response = user_service::create_user(payload, &db).await?;
    Ok(success_response(
        Some(StatusCode::CREATED),
        "User created successfully",
        serde_json::json!(user_response),
    ))
}

pub async fn login(
    payload: Json<user_dto::LoginDto>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    // Run the validation rules on the DTO
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Take the plain DTO out of the Json wrapper
    let payload = payload.into_inner();
    let user_response = user_service::login(payload, &db).await?;
    Ok(success_response(
        Some(StatusCode::OK),
        "User logged in successfully",
        serde_json::json!(user_response),
    ))
}

pub async fn get_profile(auth: AuthUser) -> Result<HttpResponse, AppError> {
    // auth.user_id was pulled out of the token by AuthUser::from_request
    let user = user_service::get_profile(auth).await?;

    Ok(success_response(
        Some(StatusCode::OK),
        "User fetched successfully",
        serde_json::json!(user),
    ))
}
