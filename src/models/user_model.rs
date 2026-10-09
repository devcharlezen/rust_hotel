
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::{dtos::user_dto::CreateUserDto, utils::{password::hash_password, response::AppError}};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: DateTime,
    pub created_at: DateTime,
    pub edited_at: DateTime,
}

impl TryFrom<CreateUserDto> for User {
    type Error = AppError;

    fn try_from(user: CreateUserDto) -> Result<Self, AppError> {
        Ok(Self {
            _id: ObjectId::new(),
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: hash_password(&user.password)?,
            date_of_birth: DateTime::from_millis(
                user.date_of_birth
                    .checked_mul(1000)
                    .ok_or(AppError::BadRequest("Invalid date of birth".to_string()))?,
            ),
            created_at: DateTime::now(),
            edited_at: DateTime::now(),
        })
    }
}
