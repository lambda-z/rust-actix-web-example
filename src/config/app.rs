use actix_web::{web};
use crate::api::{ping_controller, user_controller};


pub fn config(cfg: &mut web::ServiceConfig) {

    // General api
    cfg.service(
        web::scope("/api")
            // Input the callback function from a module.
            .service(web::resource("/users/{user_id}").to(user_controller::retrieve_user))
    );

    // Health check
    cfg.service(
        web::scope("")
            // This is a health check endpoint.
            .service(web::resource("/ping").to(ping_controller::ping))
    );
}
