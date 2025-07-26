use axum::{Json, response::IntoResponse, extract::State};
use crate::request_inputs::CreateUserInput;
use crate::request_outputs::CreateUserOutput;
use std::sync::{Arc, Mutex};
use store::store::Store;
use crate::auth::generate_jwt;

pub async fn signup_route(
    State(store): State<Arc<Mutex<Store>>>,
    Json(data): Json<CreateUserInput>,
) -> impl IntoResponse {
    let mut store = match store.lock() {
        Ok(s) => s,
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB lock error".to_string()).into_response(),
    };

    match store.sign_up(data.username.clone(), data.password, data.email.clone()) {
        Ok(user) => {
            // Generate JWT token after user is created
            let secret = "your-secret-key"; // you can move this to config/env later
            let token = match generate_jwt(user.id.to_string(), secret) {
                Ok(t) => t,
                Err(e) => {
                    return (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Token generation error: {}", e)
                    ).into_response();
                }
            };

            let response = CreateUserOutput {
                id: user.id.to_string(),
                token,
            };

            Json(response).into_response()
        },
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Signup error: {}", e)
        ).into_response(),
    }
}
