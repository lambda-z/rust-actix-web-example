
pub(crate) struct Settings {
    pub service_port: u16,
    pub mongo_url: String,
}


impl Settings {
    pub fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        Self {
            service_port: std::env::var("PORT").unwrap_or_else(
                |_| "9090".to_string()
            ).parse().expect("PORT must be a number"),

            mongo_url: "mongodb://
            localhost:27018".to_string(),
        }
    }
}
