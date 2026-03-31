use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::roles;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::pg::Pg))] // 💡 ใส่ตัวนี้ตามที่ Error แนะนำเพื่อช่วยเช็ก Type
pub struct Role {
    pub id: String, 
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}