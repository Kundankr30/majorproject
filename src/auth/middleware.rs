use axum::{
    extract::State,
    http::{Request, header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use crate::auth::verify_token;
use crate::models::{User, user};

pub async fn auth_middleware<B>(
    State(db): State<DatabaseConnection>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    if let Some(token) = auth_header {
        match verify_token(&token) {
            Ok(claims) => {
                // Verify user exists in database
                if let Ok(Some(_user)) = User::find()
                    .filter(user::Column::Id.eq(uuid::Uuid::parse_str(&claims.sub).unwrap()))
                    .one(&db)
                    .await
                {
                    request.extensions_mut().insert(claims);
                    Ok(next.run(request).await)
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
} 