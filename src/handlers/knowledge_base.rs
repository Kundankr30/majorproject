use axum::{
    extract::{Path, Query, State, Extension},
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
use serde_json::Value;

use crate::models::{KnowledgeBase, knowledge_base};
use crate::auth::Claims;

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleQuery {
    pub category: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Value,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub async fn create_article(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<ArticleResponse>, StatusCode> {
    let user_id = uuid::Uuid::new_v4();
    let article_id = Uuid::new_v4();
    let now = Utc::now();

    let tags_json = serde_json::to_value(payload.tags).map_err(|_| StatusCode::BAD_REQUEST)?;

    let article = knowledge_base::ActiveModel {
        id: Set(article_id),
        title: Set(payload.title),
        content: Set(payload.content),
        category: Set(payload.category),
        tags: Set(tags_json),
        created_by: Set(user_id),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let article = article.insert(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ArticleResponse {
        id: article.id,
        title: article.title,
        content: article.content,
        category: article.category,
        tags: article.tags,
        created_by: article.created_by,
        created_at: article.created_at,
        updated_at: article.updated_at,
    }))
}

pub async fn get_article(
    State(db): State<DatabaseConnection>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<ArticleResponse>, StatusCode> {
    let article = KnowledgeBase::find_by_id(article_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ArticleResponse {
        id: article.id,
        title: article.title,
        content: article.content,
        category: article.category,
        tags: article.tags,
        created_by: article.created_by,
        created_at: article.created_at,
        updated_at: article.updated_at,
    }))
}

pub async fn update_article(
    State(db): State<DatabaseConnection>,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<UpdateArticleRequest>,
) -> Result<Json<ArticleResponse>, StatusCode> {
    let article = KnowledgeBase::find_by_id(article_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut article: knowledge_base::ActiveModel = article.into();
    
    if let Some(title) = payload.title {
        article.title = Set(title);
    }
    if let Some(content) = payload.content {
        article.content = Set(content);
    }
    if let Some(category) = payload.category {
        article.category = Set(category);
    }
    if let Some(tags) = payload.tags {
        let tags_json = serde_json::to_value(tags).map_err(|_| StatusCode::BAD_REQUEST)?;
        article.tags = Set(tags_json);
    }
    
    article.updated_at = Set(Utc::now());

    let article = article.update(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ArticleResponse {
        id: article.id,
        title: article.title,
        content: article.content,
        category: article.category,
        tags: article.tags,
        created_by: article.created_by,
        created_at: article.created_at,
        updated_at: article.updated_at,
    }))
}

pub async fn delete_article(
    State(db): State<DatabaseConnection>,
    Path(article_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let article = KnowledgeBase::find_by_id(article_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    article.delete(&db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_articles(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ArticleQuery>,
) -> Result<Json<Vec<ArticleResponse>>, StatusCode> {
    let mut condition = Condition::all();

    if let Some(category) = query.category {
        condition = condition.add(knowledge_base::Column::Category.eq(category));
    }
    if let Some(search) = query.search {
        condition = condition.add(
            knowledge_base::Column::Title.contains(&search)
                .or(knowledge_base::Column::Content.contains(&search))
        );
    }

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50);
    let offset = (page - 1) * limit;

    let articles = KnowledgeBase::find()
        .filter(condition)
        .order_by_desc(knowledge_base::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let responses: Vec<ArticleResponse> = articles
        .into_iter()
        .map(|article| ArticleResponse {
            id: article.id,
            title: article.title,
            content: article.content,
            category: article.category,
            tags: article.tags,
            created_by: article.created_by,
            created_at: article.created_at,
            updated_at: article.updated_at,
        })
        .collect();

    Ok(Json(responses))
} 