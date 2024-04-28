
pub(crate) struct Settings {
    pub port: u16,
    pub host: String,
    pub db: String,
}


impl Settings {
    pub fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        Self {
            port: std::env::var("PORT").unwrap_or_else(
                |_| "9090".to_string()
            ).parse().expect("PORT must be a number"),

            host: "127.0.0.1".to_string(),
            db: "mongodb://
            localhost:27018".to_string(),
        }
    }
}