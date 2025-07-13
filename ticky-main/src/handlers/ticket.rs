use axum::{extract::State, Json as AxumJson};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::ticket::{CreateTicket, Ticket};
use crate::models::user::ApiResponse;

pub async fn create_ticket_handler(
    State(pool): State<PgPool>,
    AxumJson(payload): AxumJson<CreateTicket>,
) -> AxumJson<ApiResponse> {
    let _ = sqlx::query!(
        "INSERT INTO tickets (id, title, description, priority, created_by, assigned_to)
         VALUES ($1, $2, $3, $4, $5, $6)",
        Uuid::new_v4(),
        payload.title,
        payload.description,
        payload.priority.unwrap_or("medium".into()),
        payload.created_by,
        payload.assigned_to
    )
    .execute(&pool)
    .await
    .unwrap();

    AxumJson(ApiResponse { status: "Ticket Created".into() })
}

pub async fn get_tickets_handler(State(pool): State<PgPool>) -> AxumJson<Vec<Ticket>> {
    let tickets = sqlx::query_as!(Ticket, "SELECT * FROM tickets ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap();

    AxumJson(tickets)
}
