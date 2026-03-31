use std::env;
use diesel::pg::PgConnection;
use diesel::{ r2d2::{ConnectionManager, Pool}};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    // 1. โหลดค่าจากไฟล์ .env
    dotenvy::dotenv().ok();

    // 2. ดึงค่า DATABASE_URL ถ้าหาไม่เจอให้หยุดทำงาน (panic) เพราะ DB จำเป็นมาก
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // 3. สร้าง Connection Manager สำหรับ PostgreSQL
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // 4. สร้าง Pool โดยใช้ Connection Manager
    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}