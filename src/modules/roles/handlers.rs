use axum::{extract::State, Json};
use axum::http::StatusCode;
use crate::config::database::DbPool;
use crate::error::ErrorResponse;
use crate::utils::jwt::Claims;
use super::{dtos::{CreateRoleRequest, RoleResponse}, services};

#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "Roles",
    security(("bearer_auth" = [])), // 💡 บังคับใส่ Token
    request_body = CreateRoleRequest,
    responses(
        (status = 201, description = "Role created", body = RoleResponse),
        (status = 403, description = "Forbidden - Not super admin", body = ErrorResponse)
    )
)]
pub async fn create_role(
    State(pool): State<DbPool>,
    claims: Claims, // 💡 ยามเฝ้าประตูตรวจ Token
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<RoleResponse>), (StatusCode, Json<ErrorResponse>)> {
    // โยน role_id จาก Token ไปให้ Service เช็คว่าเป็น SA ไหม
    match services::create_role(&pool, claims.role_id, payload).await {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err((status, err)) => Err((status, Json(err))),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "Roles",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "List of all roles", body = [RoleResponse]),
        (status = 403, description = "Forbidden - Not super admin", body = ErrorResponse)
    )
)]
pub async fn get_all_roles(
    State(pool): State<DbPool>,
    claims: Claims,
) -> Result<Json<Vec<RoleResponse>>, (StatusCode, Json<ErrorResponse>)> {
    match services::get_all_roles(&pool, claims.role_id).await {
        Ok(res) => Ok(Json(res)),
        Err((status, err)) => Err((status, Json(err))),
    }
}