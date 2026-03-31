use serde::Serialize;
use utoipa::ToSchema;
use crate::{ modules::auth::dtos::UserResponse, utils::pagination::PaginatedResponse };

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListItem {
    pub id: String,
    pub email: String,
    pub role_name: String, // เราจะส่งชื่อ Role ไปให้ Frontend แทน ID เพื่อให้อ่านง่าย
    pub created_at: String,
}

pub type UserListResponse = PaginatedResponse<UserListItem>;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfileResponse {
    pub status: u16,
    pub message: String,
    pub data: UserResponse,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct UpdateUserRoleRequest {
    #[schema(example = "ro_admin")]
    pub role_id: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct UserDetailResponse {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct FullUserProfileResponse {
    pub id: String,
    pub email: String,
    pub role_id: String,
    pub created_at: String,
    pub details: UserDetailResponse,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct ProfileWrapperResponse {
    pub status: u16,
    pub message: String,
    pub data: FullUserProfileResponse,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct UpdateActiveStatusRequest {
    pub is_active: bool,
}