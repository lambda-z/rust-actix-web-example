use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}