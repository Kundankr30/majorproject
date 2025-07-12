use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, QueryOrder,
    Condition, ModelTrait, QuerySelect
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Ticket, ticket};
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub description: String,
    pub priority: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTicketRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TicketQuery {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub subject: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub assigned_to: Option<Uuid>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub async fn create_ticket(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<Json<TicketResponse>, StatusCode> {
    // For now, use a placeholder user_id. We'll add proper auth later
    let user_id = uuid::Uuid::new_v4();
    let ticket_id = Uuid::new_v4();
    let now = Utc::now();

    let ticket = ticket::ActiveModel {
        id: Set(ticket_id),
        subject: Set(payload.subject),
        description: Set(payload.description),
        status: Set("Open".to_string()),
        priority: Set(payload.priority),
        assigned_to: Set(None),
        created_by: Set(user_id),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let ticket = ticket.insert(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TicketResponse {
        id: ticket.id,
        subject: ticket.subject,
        description: ticket.description,
        status: ticket.status,
        priority: ticket.priority,
        assigned_to: ticket.assigned_to,
        created_by: ticket.created_by,
        created_at: ticket.created_at,
        updated_at: ticket.updated_at,
    }))
}

pub async fn get_ticket(
    State(db): State<DatabaseConnection>,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<TicketResponse>, StatusCode> {
    let ticket = Ticket::find_by_id(ticket_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(TicketResponse {
        id: ticket.id,
        subject: ticket.subject,
        description: ticket.description,
        status: ticket.status,
        priority: ticket.priority,
        assigned_to: ticket.assigned_to,
        created_by: ticket.created_by,
        created_at: ticket.created_at,
        updated_at: ticket.updated_at,
    }))
}

pub async fn update_ticket(
    State(db): State<DatabaseConnection>,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<UpdateTicketRequest>,
) -> Result<Json<TicketResponse>, StatusCode> {
    let ticket = Ticket::find_by_id(ticket_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut ticket: ticket::ActiveModel = ticket.into();
    
    if let Some(subject) = payload.subject {
        ticket.subject = Set(subject);
    }
    if let Some(description) = payload.description {
        ticket.description = Set(description);
    }
    if let Some(status) = payload.status {
        ticket.status = Set(status);
    }
    if let Some(priority) = payload.priority {
        ticket.priority = Set(priority);
    }
    if let Some(assigned_to) = payload.assigned_to {
        ticket.assigned_to = Set(Some(assigned_to));
    }
    
    ticket.updated_at = Set(Utc::now());

    let ticket = ticket.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TicketResponse {
        id: ticket.id,
        subject: ticket.subject,
        description: ticket.description,
        status: ticket.status,
        priority: ticket.priority,
        assigned_to: ticket.assigned_to,
        created_by: ticket.created_by,
        created_at: ticket.created_at,
        updated_at: ticket.updated_at,
    }))
}

pub async fn delete_ticket(
    State(db): State<DatabaseConnection>,
    Path(ticket_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let ticket = Ticket::find_by_id(ticket_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    ticket.delete(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_tickets(
    State(db): State<DatabaseConnection>,
    Query(query): Query<TicketQuery>,
) -> Result<Json<Vec<TicketResponse>>, StatusCode> {
    let mut condition = Condition::all();

    if let Some(status) = query.status {
        condition = condition.add(ticket::Column::Status.eq(status));
    }
    if let Some(priority) = query.priority {
        condition = condition.add(ticket::Column::Priority.eq(priority));
    }
    if let Some(assigned_to) = query.assigned_to {
        condition = condition.add(ticket::Column::AssignedTo.eq(assigned_to));
    }
    if let Some(created_by) = query.created_by {
        condition = condition.add(ticket::Column::CreatedBy.eq(created_by));
    }

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    let offset = (page - 1) * limit;

    let tickets = Ticket::find()
        .filter(condition)
        .order_by_desc(ticket::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<TicketResponse> = tickets
        .into_iter()
        .map(|ticket| TicketResponse {
            id: ticket.id,
            subject: ticket.subject,
            description: ticket.description,
            status: ticket.status,
            priority: ticket.priority,
            assigned_to: ticket.assigned_to,
            created_by: ticket.created_by,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
        })
        .collect();

    Ok(Json(responses))
} 