use axum::Router;
use tokio::net::TcpListener; // 💡 ต้องใช้ของ tokio เท่านั้น ห้ามใช้ของ std
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 💡 ประกาศเพื่อดึงโฟลเดอร์ต่างๆ เข้ามาในระบบ (ถ้าไม่มีบรรทัดพวกนี้ Rust จะหาไฟล์ไม่เจอครับ)
pub mod config;
pub mod error;
pub mod middlewares;
pub mod modules;
pub mod schema;
pub mod utils;

#[tokio::main]
async fn main() {
    // 1. ตั้งค่าระบบ Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,ize_service_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. เริ่มต้นเชื่อมต่อ Database
    let pool = config::database::establish_connection();
    tracing::info!("✅ Database connection pool established successfully");

    // 3. สร้าง Router หลัก
    let app = Router::new()
        .nest("/api/v1", modules::api_router())
        .layer(TraceLayer::new_for_http())
        .with_state(pool); // ส่ง pool เข้าไปให้ทุก request ใช้งานได้

    // 4. กำหนด Port
    let port = "8080";
    
    // 💡 สังเกตตรงนี้ครับ เราใช้ TcpListener ของ tokio จึงสามารถ .await ได้
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    tracing::info!("🚀 Server listening on port {}", port);

    // 5. สั่งรัน Server
    axum::serve(listener, app).await.unwrap();
}