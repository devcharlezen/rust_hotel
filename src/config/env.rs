use std::env::var;

pub struct Env {
    pub database_uri: String,
    pub jwt_secret: String,
    pub api_key: String,
    pub server_port: u16,
}

impl Env {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        let database_uri = var("MONGODB_URI").expect("MONGODB_URI must be set");
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
        let api_key = var("API_KEY").expect("API_KEY must be set");
        let server_port = var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid u16");

        Env {
            database_uri,
            jwt_secret,
            server_port,
            api_key
        }
    }
}