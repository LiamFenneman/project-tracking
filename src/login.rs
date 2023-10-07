use askama::Template;
use axum::{response::{IntoResponse, Redirect}, routing::{get, post}, Router};

use crate::AppState;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

pub async fn login() -> impl IntoResponse {
    // TODO: check credentials
    // TODO: set session
    Redirect::to("/")
}

pub async fn logout() -> impl IntoResponse {
    // TODO: clear session
    Redirect::to("/login")
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(|| async { LoginPage }))
        .route("/login", post(login))
        .route("/logout", get(logout))
}
