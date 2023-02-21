use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::entities::{UserRole, UserType};

#[derive(Deserialize, Validate)]
pub struct UserDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub user_type: UserType,
    pub role: UserRole,
    pub password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct PasswordDto {
    pub user_id: Uuid,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
}
