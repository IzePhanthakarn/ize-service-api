use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// ดึง Schema ที่เราก๊อปปี้มาจาก Migration Repo เพื่อให้ Diesel รู้จักตาราง
use crate::schema::{users, user_details};

// 1. Struct สำหรับ Query/ดึงข้อมูล (Queryable)
// ลำดับของ Field ต้องเรียงให้ตรงกับตารางใน Database เป๊ะๆ!
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub role_id: String,
    pub google_id: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 2. Struct สำหรับ Insert ข้อมูลใหม่ (Insertable)
// ตัวไหนที่ Database จัดการให้ (เช่น id ที่เป็น DEFAULT, created_at) เราไม่ต้องใส่ในนี้
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub role_id: String,
    pub google_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_details)]
#[diesel(primary_key(user_id))] // 💡 ใช้ user_id เป็น PK
pub struct UserDetail {
    pub user_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub avatar_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_details)]
pub struct NewUserDetail {
    pub user_id: String,
}