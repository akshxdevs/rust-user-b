use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct SignInInput {
    pub username: String,
    pub password: String,
}
