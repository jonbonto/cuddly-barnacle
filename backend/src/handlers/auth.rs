use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::models::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<serde_json::Value>)> {
    // Validate role
    let role = body.role.as_deref().unwrap_or("student");
    let role = match role {
        "student" | "instructor" => role.to_string(),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": "role must be 'student' or 'instructor'" })),
            ))
        }
    };

    // Validate email not empty
    if body.email.trim().is_empty() || body.full_name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "email and full_name are required" })),
        ));
    }

    // Hash password
    let password_hash = hash(&body.password, DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to hash password" })),
        )
    })?;

    let id = Uuid::new_v4();
    let now = Utc::now();

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, full_name, role, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING *",
    )
    .bind(id)
    .bind(&body.email)
    .bind(&password_hash)
    .bind(&body.full_name)
    .bind(&role)
    .bind(now)
    .bind(now)
    .fetch_one(&pool)
    .await;

    let user = match result {
        Ok(u) => u,
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("users_email_key") => {
            return Err((
                StatusCode::CONFLICT,
                Json(serde_json::json!({ "error": "Email already registered" })),
            ))
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to create user" })),
            ))
        }
    };

    let token = make_token(&user).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create token" })),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user: UserResponse {
                id: user.id.to_string(),
                email: user.email,
                full_name: user.full_name,
                role: user.role,
            },
        }),
    ))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid email or password" })),
            ))
        }
    };

    let valid = verify(&body.password, &user.password_hash).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to verify password" })),
        )
    })?;

    if !valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid email or password" })),
        ));
    }

    let token = make_token(&user).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Failed to create token" })),
        )
    })?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            full_name: user.full_name,
            role: user.role,
        },
    }))
}

pub async fn profile(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<Claims>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Invalid token" })),
        )
    })?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
        })?;

    match user {
        Some(u) => Ok(Json(UserResponse {
            id: u.id.to_string(),
            email: u.email,
            full_name: u.full_name,
            role: u.role,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "User not found" })),
        )),
    }
}

pub async fn logout() -> StatusCode {
    // JWT is stateless; the client should discard the token.
    StatusCode::NO_CONTENT
}

fn make_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiry = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiry,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
