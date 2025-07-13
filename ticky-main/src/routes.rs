use axum::{Router, routing::{post, get}};
use sqlx::PgPool;

use crate::handlers::{auth::{register_handler, login_handler}, ticket::{create_ticket_handler, get_tickets_handler}};

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/tickets", post(create_ticket_handler).get(get_tickets_handler))
        .with_state(pool)
}
