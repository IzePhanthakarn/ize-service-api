use axum::{Json, extract::{Path, Query, State}};
use axum::http::StatusCode;
use crate::{config::database::DbPool, modules::users::dtos::UpdateUserRoleRequest};
use crate::error::ErrorResponse;
use crate::modules::auth::dtos::UserResponse; use crate::modules::users::dtos::{UserListResponse, UserProfileResponse};
use crate::modules::users::repository;
// 💡 ขอยืม DTO ของ Auth มาใช้ตอบกลับ
use crate::utils::jwt::Claims; // 💡 ดึงตัวแทน Middleware มาใช้
use crate::utils::time::STANDARD_DATETIME_FORMAT;
use super::services;
use crate::utils::pagination::{PaginationParams};


#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Get profile successful", body = UserProfileResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_profile(
    State(pool): State<DbPool>,
    claims: Claims, 
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<ErrorResponse>)> { // 💡 เปลี่ยน Return Type ตรงนี้
    
    let mut conn = pool.get().map_err(|_| {
        let err = ErrorResponse { status: 500, error: "Database connection failed".to_string() };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(err))
    })?;

    match repository::get_user_by_id(&mut conn, claims.sub) {
        Ok(user) => {
            // 💡 ประกอบร่างข้อมูล User
            let user_data = UserResponse {
                id: user.id,
                email: user.email,
                role_id: user.role_id,
                created_at: user.created_at
                    .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap())
                    .format(STANDARD_DATETIME_FORMAT)
                    .to_string(),
            };

            // 💡 ห่อด้วย Wrapper ตามมาตรฐาน
            Ok(Json(UserProfileResponse {
                status: 200,
                message: "Profile retrieved successfully".to_string(),
                data: user_data,
            }))
        }
        Err(_) => {
            let err = ErrorResponse { status: 404, error: "User not found".to_string() };
            Err((StatusCode::NOT_FOUND, Json(err)))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(PaginationParams), // 💡 บอก Scalar ว่ามี query params นะ
    responses((status = 200, body = UserListResponse))
)]
pub async fn list_users(
    State(pool): State<DbPool>,
    claims: Claims,
    Query(params): Query<PaginationParams>, // 💡 รับค่าจาก ?page=1&page_size=10
) -> Result<Json<UserListResponse>, (StatusCode, Json<ErrorResponse>)> {
    match services::get_users_by_permission(&pool, claims.role_id, params).await {
        Ok(res) => Ok(Json(res)),
        Err((s, e)) => Err((s, Json(e))),
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/role",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "User ID to update")
    ),
    request_body = UpdateUserRoleRequest,
    responses(
        (status = 200, description = "User role updated successfully", body = UserProfileResponse),
        // 💡 อัปเดตคำอธิบายให้ชัดเจน
        (status = 403, description = "Forbidden - Only Super Admin allowed", body = ErrorResponse)
    )
)]
pub async fn update_user_role(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>, // 💡 รับ ID จาก URL
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    match services::update_user_role(&pool, claims.role_id, id, payload).await {
        Ok(res) => {
            Ok(Json(UserProfileResponse {
                status: 200,
                message: "User role updated successfully".to_string(),
                data: res,
            }))
        },
        Err((s, e)) => Err((s, Json(e))),
    }
}