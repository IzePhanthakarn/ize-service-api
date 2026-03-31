use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::{ modules::auth::dtos::UserResponse, utils::pagination::PaginatedResponse };

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListItem {
    pub id: Uuid,
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
    #[schema(example = "dcaa01da-3a57-44f4-aaa1-d08fbab99a7d")]
    pub role_id: Uuid,
}