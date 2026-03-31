use crate::config::database::DbPool;
use crate::error::ErrorResponse;
use crate::modules::roles::{dtos::{CreateRoleRequest, RoleResponse}, models::NewRole, repository};
use crate::utils::time::STANDARD_DATETIME_FORMAT;
use axum::http::StatusCode;

// 💡 Helper Function สำหรับตรวจสอบว่าเป็น Super Admin หรือไม่
fn check_super_admin(role_id: String) -> Result<(), (StatusCode, ErrorResponse)> {

    if role_id != "ro_super_admin" {
        return Err((StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "Requires super admin privileges".to_string() }));
    }
    Ok(())
}

// 💡 Logic สร้าง Role ใหม่
pub async fn create_role(pool: &DbPool, user_role_id: String, req: CreateRoleRequest) -> Result<RoleResponse, (StatusCode, ErrorResponse)> {
    // 1. ตรวจสอบสิทธิ์ก่อนเลย!
    check_super_admin( user_role_id)?;

    let mut conn = pool.get().unwrap();
    let new_role = NewRole { name: req.name.to_lowercase(), description: req.description }; // บังคับให้ชื่อ Role เป็นตัวเล็กทั้งหมดเพื่อความเป๊ะ

    // 2. สั่งสร้างลง Database
    let role = repository::create_role(&mut conn, &new_role).map_err(|_| {
        (StatusCode::BAD_REQUEST, ErrorResponse { status: 400, error: "Role name already exists".to_string() })
    })?;

    Ok(RoleResponse {
        id: role.id,
        name: role.name,
        created_at: role.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
    })
}

// 💡 Logic ดึง Role ทั้งหมด
pub async fn get_all_roles(pool: &DbPool, user_role_id: String) -> Result<Vec<RoleResponse>, (StatusCode, ErrorResponse)> {
    check_super_admin(user_role_id)?;

    let mut conn = pool.get().unwrap();
    let roles = repository::get_all_roles(&mut conn).unwrap_or_default();

    let response = roles.into_iter().map(|role| RoleResponse {
        id: role.id,
        name: role.name,
        created_at: role.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
    }).collect();

    Ok(response)
}