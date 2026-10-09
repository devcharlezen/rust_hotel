use mongodb::{Client, Collection};

use crate::{config::env::Env, models::{booking_model::Booking, room_model::Room, user_model::User}};

pub struct Database {
    pub bookings: Collection<Booking>,
    pub rooms: Collection<Room>,
    pub users: Collection<User>,
}

impl Database {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(Env::from_env().database_uri).await?;
        let db = client.database("hotel_project");
        println!("Database connected successfully");
        Ok(Self {
            bookings: db.collection("bookings"),
            rooms: db.collection("rooms"),
            users: db.collection("users"),
        })
    }
}
