use axum::{extract::State, Json as AxumJson};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{RegisterPayload, LoginPayload, ApiResponse};

pub async fn register_handler(
    State(pool): State<PgPool>,
    AxumJson(payload): AxumJson<RegisterPayload>,
) -> AxumJson<ApiResponse> {
    let hashed = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (id, email, password, role) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        payload.email,
        hashed,
        payload.role
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => AxumJson(ApiResponse { status: "OK".into() }),
        Err(e) => {
            eprintln!("Register error: {}", e);
            AxumJson(ApiResponse { status: "ERROR".into() })
        }
    }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    AxumJson(payload): AxumJson<LoginPayload>,
) -> AxumJson<ApiResponse> {
    let user = sqlx::query!(
        "SELECT password FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match user {
        Some(user_record) => {
            if bcrypt::verify(&payload.password, &user_record.password).unwrap() {
                AxumJson(ApiResponse { status: "OK".into() })
            } else {
                AxumJson(ApiResponse { status: "Invalid credentials".into() })
            }
        }
        None => AxumJson(ApiResponse { status: "Invalid credentials".into() }),
    }
}
