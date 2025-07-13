use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;

use crate::handlers::{auth, tickets, comments, knowledge_base};

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
       
        .route("/tickets", get(tickets::list_tickets))
        .route("/tickets", post(tickets::create_ticket))
        .route("/tickets/:id", get(tickets::get_ticket))
        .route("/tickets/:id", put(tickets::update_ticket))
        .route("/tickets/:id", delete(tickets::delete_ticket))
        .route("/tickets/:id/comments", get(comments::get_ticket_comments))
        .route("/tickets/:id/comments", post(comments::create_comment))
        .route("/knowledge-base", get(knowledge_base::list_articles))
        .route("/knowledge-base", post(knowledge_base::create_article))
        .route("/knowledge-base/:id", get(knowledge_base::get_article))
        .route("/knowledge-base/:id", put(knowledge_base::update_article))
        .route("/knowledge-base/:id", delete(knowledge_base::delete_article))
        .with_state(db)
} 