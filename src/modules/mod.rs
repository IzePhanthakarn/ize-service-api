pub mod auth;
pub mod roles;
pub mod system_services;
pub mod users;

// ประกาศ router module ที่เราเพิ่งสร้าง
pub mod router;

// Re-export api_router ออกไป เพื่อให้ main.rs เรียกใช้ง่ายๆ แบบ modules::api_router()
pub use router::api_router;