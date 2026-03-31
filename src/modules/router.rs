use axum::{response::{Html, IntoResponse}, routing::get, Json, Router};
use serde_json::{json, Value};
use utoipa::{Modify, OpenApi, openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}};
use crate::utils::time::now_thai_string;

// 💡 1. สร้างตัวปรับแต่งเพื่อให้หน้า Document รู้จักปุ่ม Authorize (Bearer Token)
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

// 1. สร้าง Struct สำหรับเก็บข้อมูล OpenAPI Document
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        crate::modules::auth::handlers::register,
        crate::modules::auth::handlers::login,
        crate::modules::users::handlers::get_profile,
        crate::modules::users::handlers::list_users,
        crate::modules::users::handlers::update_user_role,
        crate::modules::users::handlers::update_profile,
        crate::modules::users::handlers::set_user_status,
        crate::modules::roles::handlers::get_all_roles,
        crate::modules::roles::handlers::create_role,
    ),
    components(
        schemas(
            crate::modules::auth::dtos::RegisterRequest,
            crate::modules::auth::dtos::UserResponse,
            crate::modules::auth::dtos::RegisterResponse,
            crate::modules::auth::dtos::LoginRequest,
            crate::modules::auth::dtos::AuthResponse,
            crate::modules::users::dtos::UserProfileResponse,
            crate::modules::users::dtos::UserListResponse,
            crate::modules::users::dtos::UserListItem,
            crate::modules::users::dtos::UpdateProfileRequest,
            crate::modules::users::dtos::UpdateUserRoleRequest,
            crate::modules::users::dtos::FullUserProfileResponse,
            crate::modules::users::dtos::UserDetailResponse,
            crate::modules::roles::dtos::CreateRoleRequest,
            crate::modules::roles::dtos::RoleResponse,
            crate::error::ErrorResponse
        )
    ),
    modifiers(&SecurityAddon),
    tags((name = "Health", description = "System health check endpoints"))
)]
pub struct ApiDoc;

// 2. ฟังก์ชันหลักสำหรับรวม Route ของทุก Module
pub fn api_router() -> Router<crate::config::database::DbPool> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api-docs/openapi.json", get(serve_openapi_json))
        .route("/docs", get(serve_scalar_ui))
        
        .nest("/auth", crate::modules::auth::router::routes()) 
        .nest("/users", crate::modules::users::router::routes())
        .nest("/roles", crate::modules::roles::router::routes())
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