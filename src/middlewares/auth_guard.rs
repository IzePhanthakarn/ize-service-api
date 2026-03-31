use axum::{
    extract::FromRequestParts, // 💡 ลบ async_trait ออกจากตรงนี้
    http::{request::Parts, StatusCode},
    Json,
};
use crate::error::ErrorResponse;
use crate::utils::jwt::{verify_token, Claims};

// 💡 ลบ #[async_trait] ออกไปได้เลย!
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    // 💡 เขียน async fn ตรงๆ ได้เลยแบบไม่ต้องพึ่ง Macro แล้ว
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization").and_then(|value| value.to_str().ok());

        match auth_header {
            Some(auth_header) if auth_header.starts_with("Bearer ") => {
                let token = auth_header.trim_start_matches("Bearer ");
                
                match verify_token(token) {
                    Ok(claims) => Ok(claims),
                    Err(_) => {
                        let err = ErrorResponse { status: 401, error: "Invalid or expired token".to_string() };
                        Err((StatusCode::UNAUTHORIZED, Json(err)))
                    }
                }
            }
            _ => {
                let err = ErrorResponse { status: 401, error: "Missing authorization header".to_string() };
                Err((StatusCode::UNAUTHORIZED, Json(err)))
            }
        }
    }
}