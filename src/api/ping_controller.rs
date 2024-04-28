use actix_web::{HttpResponse};
use crate::service::ping_service::{PingService, PingServiceTrait};

/// #[get("/ping")]注解不支持方法级别的注解
/// 仅支持函数级别的注解
/// #[get("/ping")]
pub(crate) async fn ping() -> HttpResponse {
    let mut ping_service = PingService {
        state: "pong pong pong".to_string(),
    };
    HttpResponse::Ok().body(ping_service.ping().await)
}
