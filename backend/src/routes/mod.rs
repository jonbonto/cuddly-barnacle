use axum::{middleware, routing::{get, post}, Router};
use sqlx::PgPool;

use crate::handlers;
use crate::middleware::auth::auth_middleware;

pub fn api_router() -> Router<PgPool> {
    let protected = Router::new()
        .route("/auth/profile", get(handlers::auth::profile))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/courses", get(handlers::health::health_check))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .merge(protected)
}
