use crate::config::database::DbPool;
use crate::error::ErrorResponse;
use crate::modules::auth::dtos::UserResponse;
use crate::modules::users::dtos::{FullUserProfileResponse, UpdateProfileRequest, UpdateUserRoleRequest, UserDetailResponse, UserListItem, UserListResponse}; // 💡 ดึง DTO สำหรับ List Users มาใช้
use crate::modules::roles::repository as role_repo;
use crate::modules::users::repository;
use crate::utils::pagination::{PaginationParams, calculate_meta};
use crate::utils::time::STANDARD_DATETIME_FORMAT;
use axum::http::StatusCode;

pub async fn get_users_by_permission(
    pool: &DbPool, 
    user_role_id: String,
    params: PaginationParams // 💡 รับ params เข้ามา
) -> Result<UserListResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // 1. ตั้งค่า Default สำหรับ Pagination
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    // 2. เรียก Repo พร้อมส่ง limit/offset
    let (users_data, total_items) = match user_role_id.as_str() {
        "ro_super_admin" => repository::get_all_users_with_roles(&mut conn, page_size, offset),
        "ro_admin" => repository::get_users_only(&mut conn, page_size, offset),
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
    admin_role_id: String, // ID ของคนส่ง Request
    target_user_id: String, // ID ของ User ที่จะถูกเปลี่ยน
    req: UpdateUserRoleRequest,
) -> Result<UserResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    if admin_role_id != "ro_super_admin" {
        return Err((StatusCode::FORBIDDEN, ErrorResponse { 
            status: 403, 
            error: "Only Super Admin can change user roles".into() 
        }));
    }

    // 3. ตรวจสอบว่า Role ID ใหม่ที่ส่งมามีอยู่จริงในระบบไหม
    let _exists = role_repo::get_role_name_by_id(&mut conn, &req.role_id) 
    .map_err(|_| (StatusCode::BAD_REQUEST, ErrorResponse { status: 400, error: "Invalid role ID".into() }))?;

    // 4. ทำการอัปเดตลง Database
    let updated_user = repository::update_user_role(&mut conn, &target_user_id, req.role_id)
    .map_err(|_| (StatusCode::NOT_FOUND, ErrorResponse { status: 404, error: "User not found".into() }))?;

    Ok(UserResponse {
        id: updated_user.id,
        email: updated_user.email,
        role_id: updated_user.role_id,
        created_at: updated_user.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
    })
}

pub async fn get_my_profile(
    pool: &DbPool,
    user_id: String,
) -> Result<FullUserProfileResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // 💡 ใช้ฟังก์ชันที่ทำ Join ไว้ (users inner_join user_details)
    let (user, detail) = repository::get_user_full_profile(&mut conn, &user_id)
        .map_err(|_| (StatusCode::NOT_FOUND, ErrorResponse { status: 404, error: "User profile not found".into() }))?;

    Ok(FullUserProfileResponse {
        id: user.id,
        email: user.email,
        role_id: user.role_id,
        created_at: user.created_at.format(STANDARD_DATETIME_FORMAT).to_string(),
        details: UserDetailResponse {
            first_name: detail.first_name,
            last_name: detail.last_name,
            phone_number: detail.phone_number,
            avatar_url: detail.avatar_url,
        },
    })
}

pub async fn update_profile(
    pool: &DbPool,
    actor_id: String,      // ID ของคนกดส่ง (จาก Token)
    actor_role: String,    // Role ของคนกดส่ง (จาก Token)
    target_id: String,     // ID ของ User ที่จะถูกแก้ (จาก URL)
    req: UpdateProfileRequest,
) -> Result<UserDetailResponse, (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // --- 💡 Logic การเช็คสิทธิ์ ---
    let mut can_edit = false;

    if actor_id == target_id {
        // 1. แก้ของตัวเอง -> ได้ทุกคน
        can_edit = true;
    } else if actor_role == "ro_super_admin" {
        // 2. เป็น Super Admin -> แก้ได้ทุกคน
        can_edit = true;
    } else if actor_role == "ro_admin_shop" || actor_role == "ro_admin_company" {
        // 3. เป็น Admin -> ต้องเช็คว่าเป้าหมายไม่ใช่ Admin หรือ SA (แก้ได้เฉพาะ user ทั่วไป)
        let target_user = repository::get_user_by_id(&mut conn, &target_id)
            .map_err(|_| (StatusCode::NOT_FOUND, ErrorResponse { status: 404, error: "Target user not found".into() }))?;
        
        if target_user.role_id == "ro_user" {
            can_edit = true;
        }
    }

    if !can_edit {
        return Err((StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "You don't have permission to edit this profile".into() }));
    }
    // ---------------------------

    let updated = repository::update_user_details(&mut conn, &target_id, req)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "Update failed".into() }))?;

    Ok(UserDetailResponse {
        first_name: updated.first_name,
        last_name: updated.last_name,
        phone_number: updated.phone_number,
        avatar_url: updated.avatar_url,
    })
}

pub async fn set_user_active_status(
    pool: &DbPool,
    actor_role: String,
    target_id: String,
    status: bool,
) -> Result<(), (StatusCode, ErrorResponse)> {
    let mut conn = pool.get().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, ErrorResponse { status: 500, error: "DB Error".into() }))?;

    // เช็คสิทธิ์: เฉพาะ Admin ขึ้นไป
    if actor_role != "ro_super_admin" && actor_role != "ro_admin" {
        return Err((StatusCode::FORBIDDEN, ErrorResponse { status: 403, error: "Only admins can change active status".into() }));
    }

    repository::update_user_active_status(&mut conn, &target_id, status)
        .map_err(|_| (StatusCode::NOT_FOUND, ErrorResponse { status: 404, error: "User not found".into() }))?;

    Ok(())
}