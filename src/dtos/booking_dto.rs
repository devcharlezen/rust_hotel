use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateBookingDto {
    #[validate(length(min = 1, message = "User id cannot be empty"))]
    pub user: String,
    #[validate(length(min = 1, message = "Room id cannot be empty"))]
    pub room: String,
    #[validate(range(min = 0, message = "Check-in time must be a valid timestamp"))]
    pub checkin_time: i64,
    #[validate(range(min = 0, message = "Check-out time must be a valid timestamp"))]
    pub checkout_time: i64,
}

#[derive(Deserialize, Validate)]
pub struct GetBookingDto {
    #[validate(length(min = 1, message = "Booking id cannot be empty"))]
    pub _id: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateBookingDto {
    #[validate(length(min = 1, message = "Room id cannot be empty"))]
    pub room: Option<String>,
    #[validate(length(min = 1, message = "User id cannot be empty"))]
    pub user: Option<String>,
    #[validate(range(min = 0, message = "Check-in time must be a valid timestamp"))]
    pub checkin_time: Option<i64>,
    #[validate(range(min = 0, message = "Check-out time must be a valid timestamp"))]
    pub checkout_time: Option<i64>,
}
