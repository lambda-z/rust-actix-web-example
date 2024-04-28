use actix_web::{HttpResponse, web};
use crate::service::user_service::{UserService, UserServiceTrait};

pub(crate) async fn retrieve_user(user_id: web::Path<String>) -> HttpResponse {
    let mut user_service = UserService {};
    HttpResponse::Ok().body(user_service.get_user(&user_id).await)
}
