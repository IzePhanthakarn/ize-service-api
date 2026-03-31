use crate::config::database::DbPool;
use crate::error::ErrorResponse;
use crate::modules::auth::dtos::UserResponse;
use crate::modules::users::dtos::{UpdateUserRoleRequest, UserListItem, UserListResponse}; // 💡 ดึง DTO สำหรับ List Users มาใช้
use crate::modules::roles::repository as role_repo;
use crate::modules::users::repository;
use crate::utils::pagination::{PaginationParams, calculate_meta};
use crate::utils::time::STANDARD_DATETIME_FORMAT;
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn get_users_by_permission(
    pool: &DbPool, 
    user_role_id: Uuid,
    params: PaginationParams // 💡 รับ params เข้ามา
) -> Result<UserListResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // 1. ตั้งค่า Default สำหรับ Pagination
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let role_name = role_repo::get_role_name_by_id(&mut conn, user_role_id).map_err(|_| (StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "Role not found".into() }))?;

    // 2. เรียก Repo พร้อมส่ง limit/offset
    let (users_data, total_items) = match role_name.as_str() {
        "super_admin" => repository::get_all_users_with_roles(&mut conn, page_size, offset),
        "admin" => repository::get_users_only(&mut conn, page_size, offset),
        _ => return Err((StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "Access denied".into() })),
    }.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "Query failed".into() }))?;

    // 3. ประกอบข้อมูล
    let data = users_data.into_iter().map(|(u, r)| UserListItem {
        id: u.id,
        email: u.email,
        role_name: r.name,
        created_at: u.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
    }).collect();

    // 4. คำนวณ Meta และส่งกลับแบบมาตรฐาน
    Ok(UserListResponse {
        status: 200,
        message: "Users retrieved successfully".to_string(),
        data,
        meta: calculate_meta(total_items, page, page_size),
    })
}

pub async fn update_user_role(
    pool: &DbPool,
    admin_role_id: Uuid, // ID ของคนส่ง Request
    target_user_id: Uuid, // ID ของ User ที่จะถูกเปลี่ยน
    req: UpdateUserRoleRequest,
) -> Result<UserResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // 1. ดึงชื่อ Role ของคนทำรายการ
    let admin_role_name = role_repo::get_role_name_by_id(&mut conn, admin_role_id).map_err(|_| (StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "Role not found".into() }))?;

    // 💡 2. ปรับใหม่: บังคับว่าต้องเป็น super_admin เท่านั้น
    if admin_role_name != "super_admin" {
        return Err((StatusCode::FORBIDDEN, ErrorResponse { 
            status: 403, 
            error: "Only Super Admin can change user roles".into() 
        }));
    }

    // 3. ตรวจสอบว่า Role ID ใหม่ที่ส่งมามีอยู่จริงในระบบไหม
    let _exists = role_repo::get_role_name_by_id(&mut conn, req.role_id).map_err(|_| (StatusCode::BAD_REQUEST, ErrorResponse { status: 400, error: "Invalid role ID".into() }))?;

    // 4. ทำการอัปเดตลง Database
    let updated_user = repository::update_user_role(&mut conn, target_user_id, req.role_id).map_err(|_| (StatusCode::NOT_FOUND, ErrorResponse { status: 404, error: "User not found".into() }))?;

    Ok(UserResponse {
        id: updated_user.id,
        email: updated_user.email,
        role_id: updated_user.role_id,
        created_at: updated_user.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
    })
}