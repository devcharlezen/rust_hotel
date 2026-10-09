use actix_web::{middleware::from_fn, web};

use crate::{controllers::user_controller, utils::auth::auth_guard};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users"))
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(user_controller::login))
                .route("/create", web::post().to(user_controller::create_user)),
        )
        .service(
            web::scope("/users")
                .wrap(from_fn(auth_guard))
                .route("/me", web::get().to(user_controller::get_profile)),
        );
}
