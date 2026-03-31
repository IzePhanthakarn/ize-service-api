use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role_id: Uuid,
    pub created_at: String,
}

// 💡 อัปเดต: เพิ่ม status เข้าไปในโครงสร้าง
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    #[schema(example = 201)]
    pub status: u16,
    pub message: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub status: u16, // เผื่อไว้ให้ AuthResponse ด้วยเลยครับ
    pub message: String,
    pub access_token: String,
    pub refresh_token: String,
}

// 💡 เพิ่ม Struct สำหรับรับข้อมูลตอน Login
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "user@example.com")]
    pub email: String,
    
    #[schema(example = "password123")]
    pub password: String,
}