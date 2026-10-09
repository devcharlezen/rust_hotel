use crate::{models::user_model::User, utils::password::validate_password_strength};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    pub last_name: String,
    #[validate(range(min = 0, message = "Date of birth must be a valid timestamp"))]
    pub date_of_birth: i64,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = validate_password_strength)
    )]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct GetUserDto {
    #[validate(length(min = 1, message = "User id cannot be empty"))]
    pub _id: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 1, message = "First name cannot be empty"))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, message = "Last name cannot be empty"))]
    pub last_name: Option<String>,
    #[validate(range(min = 0, message = "Date of birth must be a valid timestamp"))]
    pub date_of_birth: Option<i64>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserPasswordDto {
    pub old_password: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = validate_password_strength)
    )]
    pub new_password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUserEmailDto {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(equal = 6, message = "OTP must be 6 characters long"))]
    pub otp: String,
}
#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"),custom(function = validate_password_strength)
)]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordDto {
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(equal = 6, message = "OTP must be 6 characters long"))]
    pub otp: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = validate_password_strength)
    )]
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub _id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt: Option<String>,
    pub created_at: i64,
    pub edited_at: i64,
}

impl UserResponse {
    pub fn from_user(user: &User, jwt: Option<String>) -> Self {
        Self {
            _id: user._id.to_hex(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            date_of_birth: user.date_of_birth.timestamp_millis() / 1000,
            jwt,
            created_at: user.created_at.timestamp_millis() / 1000,
            edited_at: user.edited_at.timestamp_millis() / 1000,
        }
    }
}
