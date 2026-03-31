use crate::modules::roles::models::Role;
use crate::schema::{roles, users};
// 💡 เพิ่มการดึง Model User (ตัวเต็ม) เข้ามาด้วย
use crate::modules::users::models::{NewUser, User}; 
use diesel::prelude::*;
use uuid::Uuid;

pub fn email_exists(conn: &mut PgConnection, email: &str) -> Result<bool, diesel::result::Error> {
    diesel::select(diesel::dsl::exists(
        users::table.filter(users::email.eq(email))
    )).get_result(conn)
}

// 💡 เปลี่ยน Return Type ให้ส่งกลับมาเป็น User ตัวเต็ม
pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, diesel::result::Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(conn) // 💡 ใช้ get_result แทน execute เพื่อดึงข้อมูลที่เพิ่งเซฟกลับมา
}

pub fn get_user_by_email(conn: &mut PgConnection, email: &str) -> Result<User, diesel::result::Error> {
    users::table
        .filter(users::email.eq(email))
        .first(conn)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, diesel::result::Error> {
    users::table
        .find(user_id) // .find() ใน Diesel คือการหาจาก Primary Key (id) ทันทีครับ
        .first(conn)
}

pub fn get_all_users_with_roles(
    conn: &mut PgConnection, 
    limit: i64, 
    offset: i64
) -> Result<(Vec<(User, Role)>, i64), diesel::result::Error> {
    // 1. นับจำนวนทั้งหมดก่อน (สำหรับคำนวณ Metadata)
    let total_count = users::table.count().get_result::<i64>(conn)?;

    // 2. ดึงข้อมูลตามหน้า
    let data = users::table
        .inner_join(roles::table)
        .limit(limit)
        .offset(offset)
        .order(users::created_at.desc()) // เรียงจากใหม่ไปเก่าเสมอเป็นมาตรฐาน
        .load::<(User, Role)>(conn)?;

    Ok((data, total_count))
}

pub fn get_users_only(
    conn: &mut PgConnection, 
    limit: i64, 
    offset: i64
) -> Result<(Vec<(User, Role)>, i64), diesel::result::Error> {
    let query = users::table.inner_join(roles::table).filter(roles::name.eq("user"));

    let total_count = query.clone().count().get_result::<i64>(conn)?;

    let data = query
        .limit(limit)
        .offset(offset)
        .order(users::created_at.desc())
        .load::<(User, Role)>(conn)?;

    Ok((data, total_count))
}

pub fn update_user_role(
    conn: &mut PgConnection, 
    user_id: Uuid, 
    new_role_id: Uuid
) -> Result<User, diesel::result::Error> {
    diesel::update(users::table.find(user_id))
        .set(users::role_id.eq(new_role_id))
        .get_result::<User>(conn)
}