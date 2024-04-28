use actix_web::{web};
use crate::api::{ping_controller, user_controller};


pub fn config(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/api")
            // 指定回调函数
            // 而非实例调用
            .service(web::resource("/ping").to(ping_controller::ping))
            .service(web::resource("/users/{user_id}").to(user_controller::retrieve_user))
    );
}
