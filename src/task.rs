use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::{info, instrument, Level};

use crate::AppState;

#[allow(dead_code)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Task {
    id: u64,
    title: String,
    description: String,
    status: Status,
    project_id: u64,
}

#[derive(Debug, Clone)]
pub enum Status {
    Open = 0,
    Closed = 1,
    Merged = 2,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        match value {
            0 => Status::Open,
            1 => Status::Closed,
            2 => Status::Merged,
            _ => panic!("invalid status"),
        }
    }
}

#[instrument(level=Level::DEBUG, skip(pool))]
pub async fn all_tasks(
    State(AppState { pool }): State<AppState>,
) -> impl IntoResponse {
    let rows = sqlx::query_as!(Task, "SELECT * FROM `task`")
        .fetch_all(&pool)
        .await
        .expect("failed to fetch tasks");
    info!("rows: {:?}", rows);

    rows.into_iter()
        .map(|row| format!("{:?}", row))
        .collect::<Vec<_>>()
        .join("\n")
}

#[instrument(level=Level::DEBUG)]
pub async fn task_by_id(
    Path((org, project, task_id)): Path<(String, String, u64)>,
) -> impl IntoResponse {
    format!("{} / {} / {}", org, project, task_id)
}
