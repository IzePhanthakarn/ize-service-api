use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total_items: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

// 💡 เพิ่ม Struct สำหรับรับ Query Params จาก URL
#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

// 💡 ฟังก์ชันช่วยคำนวณ Metadata
pub fn calculate_meta(total_items: i64, page: i64, page_size: i64) -> PaginationMeta {
    let total_pages = (total_items as f64 / page_size as f64).ceil() as i64;
    PaginationMeta {
        total_items,
        total_pages,
        current_page: page,
        page_size,
    }
}