#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct GenericResponse {
    pub status: u16,
    pub message: String,
}