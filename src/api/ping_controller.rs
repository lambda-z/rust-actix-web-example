use actix_web::{HttpResponse, web};
use log::info;
use crate::AppState;
use crate::service::ping_service::{PingService, PingServiceTrait};

pub(crate) async fn ping(context: web::Data<AppState>) -> HttpResponse {
    let mut ping_service = PingService {
        state: "pong".to_string(),
    };
    info!("port: {:?}", context.settings.service_port);
    HttpResponse::Ok().body(ping_service.ping().await)
}
