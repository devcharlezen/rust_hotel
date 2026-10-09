use crate::utils::password::validate_password_strength;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateRoomDto {
    #[validate(length(min = 1, message = "Room name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Room description cannot be empty"))]
    pub description: String,
    #[validate(range(min = 1, message = "Room capacity cannot be empty"))]
    pub capacity: u64,
    #[validate(range(min = 1, message = "Room number cannot be empty"))]
    pub number: u64,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = validate_password_strength)
    )]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct GetRoomDto {
    #[validate(length(min = 1, message = "Room number cannot be empty"))]
    pub number: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRoomDto {
    #[validate(length(min = 1, message = "Room name cannot be empty"))]
    pub name: Option<String>,
    #[validate(length(min = 1, message = "Room description cannot be empty"))]
    pub description: Option<String>,
    #[validate(range(min = 1, message = "Room capacity cannot be empty"))]
    pub capacity: Option<u64>,
    #[validate(range(min = 1, message = "Room number cannot be empty"))]
    pub number: Option<u64>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateRoomPasswordDto {
    #[validate(length(min = 1, message = "Old password cannot be empty"))]
    pub old_password: String,
    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = validate_password_strength)
    )]
    pub new_password: String,
}
