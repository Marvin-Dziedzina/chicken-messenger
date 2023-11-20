use serde::Deserialize;

#[derive(Deserialize)]
pub struct PwParam {
    pub auth_password_hash: String,
    pub user_name: String,
    pub password: String,
    pub salt: String,
}
