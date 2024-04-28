pub(crate) trait PingServiceTrait {
    async fn ping(&mut self) -> String;
}

pub(crate) struct PingService {
    pub(crate) state: String,
}
