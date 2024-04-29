use log::info;
use mongodb::Client;

struct Mongo {
    uri: String,
}

impl Mongo {
    pub fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        Self {
            uri: std::env::var("MONGO_URI").unwrap_or_else(
                |_| "mongodb://localhost:27017".to_string()
            ),
        }
    }

    async fn client(&self) -> Client {
        info!("mongo uri: {:?}", self.uri);
        Client::with_uri_str(&self.uri).await.unwrap()
    }
}