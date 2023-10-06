use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tracing::{info, instrument, Level};

use crate::AppState;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Organisation {
    id: u64,
    name: String,
}

#[instrument(level=Level::DEBUG, skip(pool))]
pub async fn all_orgs(
    State(AppState { pool }): State<AppState>,
) -> impl IntoResponse {
    let rows = sqlx::query_as!(Organisation, "SELECT * FROM `organisation`")
        .fetch_all(&pool)
        .await
        .expect("failed to fetch organisations");
    info!("rows: {:?}", rows);

    rows.into_iter()
        .map(|row| format!("{:?}", row))
        .collect::<Vec<_>>()
        .join("\n")
}

#[instrument(level=Level::DEBUG)]
pub async fn org_by_id(Path(org): Path<String>) -> impl IntoResponse {
    org
}
