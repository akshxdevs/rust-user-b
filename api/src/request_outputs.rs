use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserOutput {
    pub id: String,
    pub token: String
}
