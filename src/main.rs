use anyhow::Context;
use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, services::ServeDir, trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod org;
mod project;
mod task;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: sqlx::MySqlPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("error while loading .env")?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    "warn,tower_http=trace,project_tracking_app=debug".into()
                }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let host = option_env!("HOST")
        .and_then(|port| port.parse().ok())
        .unwrap_or_else(|| std::net::Ipv4Addr::new(0, 0, 0, 0));
    let port = option_env!("PORT")
        .and_then(|port| port.parse().ok())
        .unwrap_or(3000);
    let addr = std::net::SocketAddr::from((host, port));
    let public_path = std::env::current_dir()
        .expect("could not find current working dir")
        .join("public");

    let pool = sqlx::MySqlPool::connect(
        &std::env::var("DATABASE_URL").context("DATABASE_URL not set")?,
    )
    .await
    .context("error while connecting to database")?;

    let router = Router::new()
        .route("/", get(home))
        .route("/orgs", get(org::all_orgs))
        .nest(
            "/:org",
            Router::new()
                .route("/", get(org::org_by_id))
                .route("/projects", get(project::all_projects))
                .nest(
                    "/:project",
                    Router::new()
                        .route("/", get(project::project_by_id))
                        .route("/tasks", get(task::all_tasks))
                        .nest(
                            "/:task_id",
                            Router::new().route("/", get(task::task_by_id)),
                        ),
                ),
        )
        .nest_service("/public", ServeDir::new(public_path))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http().on_response(
                        tower_http::trace::DefaultOnResponse::new()
                            .level(tracing::Level::INFO)
                            .latency_unit(tower_http::LatencyUnit::Millis),
                    ),
                )
                .layer(CompressionLayer::new()),
        )
        .with_state(AppState { pool });

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct HomePage;

async fn home() -> impl IntoResponse {
    HomePage
}
