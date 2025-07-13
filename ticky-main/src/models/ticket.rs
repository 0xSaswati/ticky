use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub created_by: Uuid,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_by: Uuid,
    pub assigned_to: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}