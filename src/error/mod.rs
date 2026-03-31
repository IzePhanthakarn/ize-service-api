use serde::Serialize;
use utoipa::ToSchema;

// 💡 โครงสร้าง JSON มาตรฐานเวลาเกิด Error ทุกจุดในระบบ
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
}