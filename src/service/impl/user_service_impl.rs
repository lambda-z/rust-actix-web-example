use crate::service::user_service::{UserService, UserServiceTrait};

impl UserServiceTrait for UserService {
    async fn get_user(&mut self, user_id: &str) -> String {
        format!("user_id: {}", user_id)
    }
}