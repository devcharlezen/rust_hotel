use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::dtos::booking_dto::CreateBookingDto;

#[derive(Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub user: ObjectId,
    pub room: ObjectId,
    pub check_in_at: DateTime,
    pub check_out_at: DateTime,
    pub created_at: DateTime,
    pub edited_at: DateTime,
}

impl TryFrom<CreateBookingDto> for Booking {
    type Error = String;

    fn try_from(booking: CreateBookingDto) -> Result<Self, Self::Error> {
        if booking.checkout_time <= booking.checkin_time {
            return Err("Check out must be after check in".to_string());
        }
        Ok(Self {
            _id: ObjectId::new(),
            user: ObjectId::parse_str(&booking.user).map_err(|_| "Invalid user id".to_string())?,
            room: ObjectId::parse_str(&booking.room).map_err(|_| "Invalid room id".to_string())?,
            check_in_at: DateTime::from_millis(
                booking
                    .checkin_time
                    .checked_mul(1000)
                    .ok_or("Invalid check in time".to_string())?,
            ),
            check_out_at: DateTime::from_millis(
                booking
                    .checkout_time
                    .checked_mul(1000)
                    .ok_or("Invalid check out time".to_string())?,
            ),
            created_at: DateTime::now(),
            edited_at: DateTime::now(),
        })
    }
}
