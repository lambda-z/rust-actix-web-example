use crate::service::ping_service::{PingService, PingServiceTrait};

impl PingServiceTrait for PingService {
    async fn ping(&mut self) -> String {
        log::info!("pong!");
        self.state.clone()
    }
}