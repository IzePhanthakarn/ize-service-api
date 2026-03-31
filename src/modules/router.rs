use axum::{response::{Html, IntoResponse}, routing::get, Json, Router};
use serde_json::{json, Value};
use utoipa::OpenApi;
use crate::utils::time::now_thai_string;

// 1. สร้าง Struct สำหรับเก็บข้อมูล OpenAPI Document
#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    tags((name = "Health", description = "System health check endpoints"))
)]
pub struct ApiDoc;

// 2. ฟังก์ชันหลักสำหรับรวม Route ของทุก Module
pub fn api_router() -> Router<crate::config::database::DbPool> {
    Router::new()
        .route("/health", get(health_check))
        
        // 💡 เส้นทางที่ 1: สำหรับเสิร์ฟไฟล์ JSON Spec (เผื่อให้ Postman หรือ Frontend ดึงไปใช้)
        .route("/api-docs/openapi.json", get(serve_openapi_json))
        
        // 💡 เส้นทางที่ 2: สำหรับเสิร์ฟหน้าจอ Scalar UI
        .route("/docs", get(serve_scalar_ui))
}

// 3. Handler เสิร์ฟ OpenAPI JSON แบบตรงๆ
async fn serve_openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

// 4. Handler เสิร์ฟหน้า Scalar UI (ใช้ HTML ดึง Component จาก CDN)
async fn serve_scalar_ui() -> impl IntoResponse {
    let html = r#"
    <!doctype html>
    <html>
      <head>
        <title>API Documentation</title>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <style>body { margin: 0; }</style>
      </head>
      <body>
        <script id="api-reference" data-url="/api/v1/api-docs/openapi.json"></script>
        <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
      </body>
    </html>
    "#;
    
    // แปลง String เป็น HTTP Response แบบ HTML
    Html(html)
}

// 5. Health Check Handler
#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "System is healthy and running", body = Value)
    )
)]
async fn health_check() -> Json<Value> {
    let response = json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"), 
        "message": "✨ System is healthy and running",
        "timestamp": now_thai_string(),
    });

    Json(response)
}