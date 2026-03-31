use axum::{Json, extract::{Path, Query, State}};
use axum::http::StatusCode;
use crate::{config::database::DbPool, modules::users::dtos::{ProfileWrapperResponse, UpdateActiveStatusRequest, UpdateProfileRequest, UpdateUserRoleRequest, UserListResponse, UserProfileResponse}, utils::dtos::GenericResponse};
use crate::error::ErrorResponse;
// 💡 ขอยืม DTO ของ Auth มาใช้ตอบกลับ
use crate::utils::jwt::Claims; // 💡 ดึงตัวแทน Middleware มาใช้
use super::services;
use crate::utils::pagination::{PaginationParams};


#[utoipa::path(
    get,
    path = "/api/v1/users/me",
    tag = "Users",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, body = ProfileWrapperResponse),
        (status = 401, body = ErrorResponse)
    )
)]
pub async fn get_profile(
    State(pool): State<DbPool>,
    claims: Claims, // 💡 ดึง ID จาก Token มาใช้
) -> Result<Json<ProfileWrapperResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 💡 claims.sub ตอนนี้เป็น String แล้ว (ตามที่เราแก้เรื่อง ID)
    match services::get_my_profile(&pool, claims.sub).await {
        Ok(res) => Ok(Json(ProfileWrapperResponse {
            status: 200,
            message: "Success".to_string(),
            data: res,
        })),
        Err((s, e)) => Err((s, Json(e))),
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

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/profile",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "User ID to update")),
    request_body = UpdateProfileRequest,
    responses((status = 200, body = ProfileWrapperResponse))
)]
pub async fn update_profile(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileWrapperResponse>, (StatusCode, Json<ErrorResponse>)> {
    
    // 1. สั่งอัปเดตผ่าน Service
    // เราใส่ _res เพื่อบอก Rust ว่า "ฉันรู้ว่ามีค่าส่งกลับมานะ แต่ตอนนี้ยังไม่ได้ใช้"
    let _res = services::update_profile(&pool, claims.sub, claims.role_id, id.clone(), payload).await
        .map_err(|(s, e)| (s, Json(e)))?;

    // 2. ดึงข้อมูล Profile ล่าสุด (Full Profile) กลับมาเพื่อส่งให้ Frontend
    match services::get_my_profile(&pool, id).await {
        Ok(full_profile) => {
            Ok(Json(ProfileWrapperResponse {
                status: 200,
                message: "Profile updated successfully".to_string(),
                data: full_profile, // 💡 ใส่ข้อมูลที่ดึงมาใหม่ตรงนี้
            }))
        },
        Err((s, e)) => Err((s, Json(e))),
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/{id}/status",
    tag = "Users",
    security(("bearer_auth" = [])),
    params(
        ("id" = String, Path, description = "ID ของ User ที่ต้องการเปลี่ยนสถานะ")
    ),
    request_body = UpdateActiveStatusRequest,
    responses(
        (status = 200, description = "เปลี่ยนสถานะสำเร็จ"),
        (status = 403, body = ErrorResponse, description = "ไม่มีสิทธิ์ทำรายการ"),
        (status = 404, body = ErrorResponse, description = "ไม่พบ User")
    )
)]
pub async fn set_user_status(
    State(pool): State<DbPool>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<UpdateActiveStatusRequest>,
) -> Result<Json<GenericResponse>, (StatusCode, Json<ErrorResponse>)> { // 💡 เปลี่ยนตรงนี้
    
    // เรียกใช้ Service
    services::set_user_active_status(
        &pool, 
        claims.role_id, 
        id.clone(), // 💡 ใช้ clone เพราะเดี๋ยวเราจะเอา id ไปใส่ใน message
        payload.is_active
    ).await.map_err(|(s, e)| (s, Json(e)))?;

    // 💡 สร้าง Message ให้ดูดี
    let action = if payload.is_active { "activated" } else { "deactivated" };
    
    Ok(Json(GenericResponse {
        status: 200,
        message: format!("User {} has been successfully {}", id, action),
    }))
}