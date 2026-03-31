use axum::{ routing::{ get }, Router };
use crate::{ config::database::DbPool, modules::roles::handlers };

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", get(handlers::get_all_roles).post(handlers::create_role)) // 💡 ใช้ path "/" เพราะเราจะเอาไป nest เป็น /roles ทีหลัง
}
