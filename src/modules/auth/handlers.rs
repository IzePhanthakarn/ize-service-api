use axum::{extract::State, Json};
use axum::http::StatusCode;
use crate::config::database::DbPool;
use crate::modules::auth::dtos::{AuthResponse, LoginRequest};
use super::dtos::{RegisterRequest, RegisterResponse}; 
use super::services;
use crate::error::ErrorResponse; // 💡 ดึง Error มาตรฐานของเรามาใช้

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        // 💡 อัปเดต Document ให้บอกว่า Error ก็เป็น JSON นะ
        (status = 400, description = "Bad Request", body = ErrorResponse) 
    )
)]
pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), (StatusCode, Json<ErrorResponse>)> { // 💡 เปลี่ยน Type ตรงฝั่ง Err
    
    match services::register(&pool, payload).await {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => {
            // 💡 ถ้าพัง ให้ประกอบร่าง JSON Error ส่งกลับไป
            let error_json = ErrorResponse {
                status: 400,
                error: e,
            };
            Err((StatusCode::BAD_REQUEST, Json(error_json)))
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized - Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    
    match services::login(&pool, payload).await {
        Ok(res) => Ok((StatusCode::OK, Json(res))),
        Err(e) => {
            let error_json = ErrorResponse {
                status: 401, // Error ของการ Login ผิดพลาดคือ 401 Unauthorized
                error: e,
            };
            Err((StatusCode::UNAUTHORIZED, Json(error_json)))
        }
    }
}