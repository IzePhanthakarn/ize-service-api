use axum::{ routing::post, Router };
use crate::{ config::database::DbPool, modules::auth::handlers };

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}
