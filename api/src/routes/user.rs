use crate::auth::generate_jwt;
use crate::request_inputs::{CreateUserInput, SignInInput};
use crate::request_outputs::CreateUserOutput;
use axum::{extract::State, response::IntoResponse, Json};
use std::sync::{Arc, Mutex};
use store::store::Store;

pub async fn signup_route(
    State(store): State<Arc<Mutex<Store>>>,
    Json(data): Json<CreateUserInput>,
) -> impl IntoResponse {
    let mut store = match store.lock() {
        Ok(s) => s,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "DB lock error".to_string(),
            )
                .into_response();
        }
    };

    match store.sign_up(data.username.clone(), data.password, data.email.clone()) {
        Ok(user) => {
            let secret = "your-secret-key"; // Move to env/config in real app
            match generate_jwt(user.id.to_string(), secret) {
                Ok(token) => {
                    let response = CreateUserOutput {
                        id: user.id.to_string(),
                        token,
                    };
                    Json(response).into_response()
                }
                Err(e) => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Token generation error: {}", e),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Signup error: {}", e),
        )
            .into_response(),
    }
}

pub async fn signin_route(
    State(store): State<Arc<Mutex<Store>>>,
    Json(data): Json<SignInInput>,
) -> impl IntoResponse {
    let mut store = match store.lock() {
        Ok(s) => s,
        Err(_) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "DB lock error".to_string(),
            )
                .into_response();
        }
    };

    match store.sign_in(data.username.clone(), data.password.clone()) {
        Ok(Some(user)) => {
            let secret = "akshxsect34";
            match generate_jwt(user.id.to_string(), secret) {
                Ok(token) => {
                    let response = CreateUserOutput {
                        id: user.id.to_string(),
                        token,
                    };
                    Json(response).into_response()
                }
                Err(e) => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Token generation error: {}", e),
                )
                .into_response(),
            }
        }
        Ok(None) => (
            axum::http::StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        )
        .into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Signin error: {}", e),
        )
        .into_response(),
    }
}

