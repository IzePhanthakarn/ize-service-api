use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    #[schema(example = "admin_shop")]
    pub name: String,
    #[schema(example = "ผู้ดูแลร้านค้า")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
}