use crate::controllers::dto::user_dto::NewUserDTO;
use crate::data::models::user::NewUser;

impl<'a> From<&'a NewUserDTO> for NewUser<'a> {
    fn from(user_dto: &'a NewUserDTO) -> Self {
        NewUser {
            username: &user_dto.username,
            password_hash: &user_dto.password,
        }
    }
}
