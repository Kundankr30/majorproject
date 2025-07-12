use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::models::{Comment, comment};
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub is_internal: bool,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub is_internal: bool,
    pub created_at: chrono::DateTime<Utc>,
}

pub async fn create_comment(
    State(db): State<DatabaseConnection>,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<CommentResponse>, StatusCode> {
    // For now, use a placeholder user_id. We'll add proper auth later
    let user_id = uuid::Uuid::new_v4();
    let comment_id = Uuid::new_v4();
    let now = Utc::now();

    let comment = comment::ActiveModel {
        id: Set(comment_id),
        ticket_id: Set(ticket_id),
        user_id: Set(user_id),
        content: Set(payload.content),
        is_internal: Set(payload.is_internal),
        created_at: Set(now),
    };

    let comment = comment.insert(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CommentResponse {
        id: comment.id,
        ticket_id: comment.ticket_id,
        user_id: comment.user_id,
        content: comment.content,
        is_internal: comment.is_internal,
        created_at: comment.created_at,
    }))
}

pub async fn get_ticket_comments(
    State(db): State<DatabaseConnection>,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<Vec<CommentResponse>>, StatusCode> {
    // For now, show all comments. We'll add role-based filtering later
    let condition = comment::Column::TicketId.eq(ticket_id);

    let comments = Comment::find()
        .filter(condition)
        .order_by_asc(comment::Column::CreatedAt)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<CommentResponse> = comments
        .into_iter()
        .map(|comment| CommentResponse {
            id: comment.id,
            ticket_id: comment.ticket_id,
            user_id: comment.user_id,
            content: comment.content,
            is_internal: comment.is_internal,
            created_at: comment.created_at,
        })
        .collect();

    Ok(Json(responses))
} 