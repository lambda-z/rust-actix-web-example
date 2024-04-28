use actix_web::{HttpResponse};
use crate::service::ping_service::{PingService, PingServiceTrait};

pub(crate) async fn ping() -> HttpResponse {
    let mut ping_service = PingService {
        state: "pong".to_string(),
    };
    HttpResponse::Ok().body(ping_service.ping().await)
}
