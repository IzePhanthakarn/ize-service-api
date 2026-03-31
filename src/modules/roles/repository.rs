use crate::modules::users::models::User;
use crate::schema::{roles, users};
use crate::modules::roles::models::{NewRole, Role}; // 💡 ดึง Model มาใช้
use diesel::prelude::*;

// ค้นหา ID ของ Role จากชื่อ (เช่น หา id ของคำว่า "user")
pub fn get_role_id_by_name(conn: &mut PgConnection, role_name: &str) -> Result<String, diesel::result::Error> {
    roles::table
        .filter(roles::name.eq(role_name))
        .select(roles::id)
        .first(conn)
}

pub fn get_role_name_by_id(conn: &mut PgConnection, role_id: &str) -> Result<String, diesel::result::Error> {
    roles::table
        .find(role_id)
        .select(roles::name)
        .first(conn)
}

// 💡 2. ฟังก์ชันดึง Role ทั้งหมดในระบบ
pub fn get_all_roles(conn: &mut PgConnection) -> Result<Vec<Role>, diesel::result::Error> {
    roles::table.load::<Role>(conn)
}

// 💡 3. ฟังก์ชันสร้าง Role ใหม่
pub fn create_role(conn: &mut PgConnection, new_role: &NewRole) -> Result<Role, diesel::result::Error> {
    diesel::insert_into(roles::table)
        .values(new_role)
        .get_result(conn)
}

// 💡 ดึง User พร้อม Role (สำหรับ SA - เห็นทั้งหมด)
pub fn get_all_users_with_roles(conn: &mut PgConnection) -> Result<Vec<(User, Role)>, diesel::result::Error> {
    users::table
        .inner_join(roles::table)
        .load::<(User, Role)>(conn)
}

// 💡 ดึงเฉพาะ User ที่มี Role เป็น 'user' (สำหรับ Admin)
pub fn get_users_only(conn: &mut PgConnection) -> Result<Vec<(User, Role)>, diesel::result::Error> {
    users::table
        .inner_join(roles::table)
        .filter(roles::name.eq("user"))
        .load::<(User, Role)>(conn)
}