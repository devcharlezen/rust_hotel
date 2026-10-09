use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::dtos::room_dto::CreateRoomDto;

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub capacity: u64,
    pub number: u64,
    pub created_at: DateTime,
    pub edited_at: DateTime,
    pub currently_booked: bool,
    pub booking: Option<ObjectId>,
}

impl TryFrom<CreateRoomDto> for Room {
    type Error = String;

    fn try_from(room: CreateRoomDto) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: room.name,
            description: room.description,
            capacity: room.capacity,
            number: room.number,
            created_at: DateTime::now(),
            edited_at: DateTime::now(),
            currently_booked: false,
            booking: None,
        })
    }
}
