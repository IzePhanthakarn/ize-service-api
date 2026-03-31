use axum::{routing::{get,patch}, Router};
use crate::{config::database::DbPool, modules::users::handlers};

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(handlers::list_users)) // GET /api/v1/users
        .route("/me", get(handlers::get_profile))
        .route("/{id}/role", patch(handlers::update_user_role))
}