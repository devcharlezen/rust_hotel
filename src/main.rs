mod config;
mod controllers;
mod dtos;
mod models;
mod routes;
mod services;
mod utils;

use crate::{
    config::{db::Database, env::Env},
    routes::user_route::user_routes,
    utils::api_key::api_key_guard,
};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, main, middleware::from_fn, web};

#[main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
        .await
        .expect("Failed to connect to MongoDB");

    let db_data = web::Data::new(db);

    let governor = GovernorConfigBuilder::default()
        .per_second(2) // refill 2 requests per second
        .burst_size(5) // allow a short burst of 5
        .finish()
        .unwrap();

    let server = HttpServer::new(move || {
        // Built inside, so each worker makes its own
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(from_fn(api_key_guard))
            .wrap(Governor::new(&governor))
            .wrap(cors)
            .app_data(db_data.clone())
            .configure(user_routes)
    })
    .bind(format!("localhost:{}", Env::from_env().server_port))?;

    println!(
        "Server running on http://localhost:{}",
        Env::from_env().server_port
    );

    server.run().await
}
