use lazy_static::lazy_static;



pub(crate) struct Settings {
    pub service_host: String,
    pub service_port: u16,
    pub mongo_url: String,
}


impl Settings {

    fn get_env(key: String, default: String) ->Option<String> {
        Option::from(std::env::var(key)
            .unwrap_or_else(
                |_| default.to_string()
            ))
    }


    pub fn new() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        Self {
            service_host: Self::get_env(
                "SERVICE_HOST".to_string(),
                "127.0.0.1".to_string()).unwrap(),

            service_port: Self::get_env(
                "SERVICE_PORT".to_string(),
                "8080".to_string()
            ).unwrap()
             .parse()
             .expect("PORT must be a number"),

            mongo_url: Self::get_env(
                "MONGO_URI".to_string(),
                "".to_string()).unwrap(),
        }
    }
}


lazy_static! {
    pub(crate) static ref SETTINGS: Settings = Settings::new();
}