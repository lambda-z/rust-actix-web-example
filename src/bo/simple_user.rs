use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SimpleUser {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
