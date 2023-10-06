use anyhow::Context;
use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,project_tracking_app=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let port = 3000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let public_path = std::env::current_dir()
        .expect("could not find current working dir")
        .join("public");

    let api_router = Router::new().route("/hello", get(hello_sv));
    let router = Router::new()
        .route("/", get(hello))
        .nest("/api", api_router)
        .nest_service("/public", ServeDir::new(public_path));

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn hello() -> impl IntoResponse {
    HelloTemplate
}

async fn hello_sv() -> impl IntoResponse {
    "Hello, world!"
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate;
