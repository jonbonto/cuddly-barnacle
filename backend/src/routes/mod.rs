use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers;

pub fn api_router() -> Router<PgPool> {
    Router::new()
        .route("/courses", get(handlers::health::health_check))
}
