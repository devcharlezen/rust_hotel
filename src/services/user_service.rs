use crate::{
    config::db::Database,
    dtos::user_dto::{CreateUserDto, LoginDto, UserResponse},
    models::user_model::User,
    utils::{auth::AuthUser, password::compare_password, response::AppError, token::Tokens},
};
use mongodb::bson::{doc, oid::ObjectId};

pub async fn create_user(payload: CreateUserDto, db: &Database) -> Result<UserResponse, AppError> {
    // Check if the email is already taken
    let existing_user = db
        .users
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if existing_user.is_some() {
        return Err(AppError::Conflict(
            "User with this email already exists".to_string(),
        ));
    }

    let user = User::try_from(payload)?;

    // Save it
    db.users
        .insert_one(&user)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    //Reply with 201 Created, never send the password back
    Ok(UserResponse::from_user(&user, None))
}

pub async fn login(payload: LoginDto, db: &Database) -> Result<UserResponse, AppError> {
    // Check if the email is already taken
    let existing_user = db
        .users
        .find_one(doc! { "email": &payload.email })
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .ok_or(AppError::BadRequest(
            "Invalid email or password".to_string(),
        ))?;

    let is_valid_password = compare_password(&payload.password, &existing_user.password)?;

    if !is_valid_password {
        return Err(AppError::BadRequest("“Invalid email or password".to_string()));
    }

    let jwt = Tokens::generate_jwt(&UserResponse::from_user(&existing_user, None))?;

    //Reply with 201 Created, never send the password back
    Ok(UserResponse::from_user(&existing_user, Some(jwt)))
}

pub async fn get_profile(payload: AuthUser) -> Result<UserResponse, AppError> {
    Ok(payload.user)
}
