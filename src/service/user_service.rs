
pub(crate) struct UserService {

}

pub(crate) trait UserServiceTrait {
    async fn get_user(&mut self, user_id: &str) -> String;
}