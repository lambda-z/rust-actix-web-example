use crate::service::ping_service::{PingService, PingServiceTrait};

impl PingServiceTrait for PingService {
    async fn ping(&mut self) -> String {
        let pong = &self.state;
        log::info!("{}", pong);
        pong.to_string()
    }
}