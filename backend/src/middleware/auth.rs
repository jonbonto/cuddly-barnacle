use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::handlers::auth::{decode_token, Claims};

pub async fn auth_middleware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims: Claims = match decode_token(token) {
        Ok(c) => c,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
