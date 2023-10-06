use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::{info, instrument, Level};

use crate::AppState;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Project {
    id: u64,
    name: String,
    org_id: u64,
}

#[instrument(level=Level::DEBUG, skip(pool))]
pub async fn all_projects(
    State(AppState { pool }): State<AppState>,
) -> impl IntoResponse {
    let rows = sqlx::query_as!(Project, "SELECT * FROM `project`")
        .fetch_all(&pool)
        .await
        .expect("failed to fetch projects");
    info!("rows: {:?}", rows);

    rows.into_iter()
        .map(|row| format!("{:?}", row))
        .collect::<Vec<_>>()
        .join("\n")
}

#[instrument(level=Level::DEBUG)]
pub async fn project_by_id(
    Path((org, project)): Path<(String, String)>,
) -> impl IntoResponse {
    format!("{} / {}", org, project)
}
