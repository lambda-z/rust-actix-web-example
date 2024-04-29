use actix_web::{HttpResponse, web};
use log::info;
use crate::AppState;
use crate::service::user_service::{UserService, UserServiceTrait};

pub(crate) async fn retrieve_user(user_id: web::Path<String>, context: web::Data<AppState>) -> HttpResponse {
    let mut user_service = UserService {};
    info!("port: {:?}", context.settings.service_port);
    HttpResponse::Ok().body(user_service.get_user(&user_id).await)
}
