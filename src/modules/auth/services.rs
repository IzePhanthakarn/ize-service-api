use crate::config::database::DbPool;
// 💡 เปลี่ยนมาเรียกใช้ RegisterResponse และ UserResponse
use crate::modules::auth::dtos::{AuthResponse, LoginRequest, RegisterRequest, RegisterResponse, UserResponse}; 
use crate::modules::roles::repository as role_repo;
use crate::modules::users::{models::NewUser, repository as user_repo};
use crate::utils::jwt::generate_tokens;
use crate::utils::password::{hash_password, verify_password};
use crate::utils::time::STANDARD_DATETIME_FORMAT; // 💡 ดึง Format เวลาของเรามาใช้

pub async fn register(pool: &DbPool, req: RegisterRequest) -> Result<RegisterResponse, String> {
    let mut conn = pool.get().map_err(|_| "Database connection error".to_string())?;

    let exists = user_repo::email_exists(&mut conn, &req.email)
        .map_err(|_| "Database error".to_string())?;
        
    if exists {
        return Err("Email already exists".to_string());
    }

    let role_id = role_repo::get_role_id_by_name(&mut conn, "user")
        .map_err(|_| "Default role not found. Did you run migrations?".to_string())?;

    let hashed_password = hash_password(&req.password)
        .map_err(|_| "Failed to hash password".to_string())?;

    let new_user = NewUser {
        email: req.email,
        password_hash: Some(hashed_password),
        role_id,
        google_id: None,
    };

    // 💡 คราวนี้เราจะได้ตัวแปร created_user กลับมาใช้งานแล้ว
    let created_user = user_repo::create_user(&mut conn, &new_user)
        .map_err(|_| "Failed to create user".to_string())?;

    // 💡 ส่งข้อมูลกลับไปพร้อม Status 201 (Created)
    Ok(RegisterResponse {
        status: 201, // <-- เพิ่มบรรทัดนี้
        message: "User registered successfully".to_string(),
        user: UserResponse {
            id: created_user.id,
            email: created_user.email,
            role_id: created_user.role_id,
            created_at: created_user.created_at.with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).unwrap()).format(STANDARD_DATETIME_FORMAT).to_string(),
        }
    })
}

pub async fn login(pool: &DbPool, req: LoginRequest) -> Result<AuthResponse, String> {
    let mut conn = pool.get().map_err(|_| "Database connection error".to_string())?;

    // 1. ค้นหา User จาก Email
    let user = match user_repo::get_user_by_email(&mut conn, &req.email) {
        Ok(u) => u,
        // Trick: แม้จะหาอีเมลไม่เจอ เราก็ควรตอบกว้างๆ ว่า "Invalid email or password" 
        // เพื่อป้องกันไม่ให้ Hacker รู้ว่ามีอีเมลนี้อยู่ในระบบหรือเปล่า (Security Best Practice)
        Err(_) => return Err("Invalid email or password".to_string()),
    };

    // 2. ตรวจสอบว่า User มีรหัสผ่านไหม (อาจจะ Login ผ่าน Google OAuth มาก่อนเลยไม่มีรหัส)
    let password_hash = match user.password_hash {
        Some(hash) => hash,
        None => return Err("Please login using your OAuth provider".to_string()),
    };

    // 3. ตรวจสอบความถูกต้องของรหัสผ่านด้วย Argon2
    if !verify_password(&req.password, &password_hash) {
        return Err("Invalid email or password".to_string());
    }

    // 4. ตรวจสอบว่าบัญชีนี้โดนระงับการใช้งานอยู่หรือไม่
    if !user.is_active {
        return Err("This account has been disabled".to_string());
    }

    // 5. รหัสผ่านถูกต้อง! สั่งออกคู่ Token ให้เลย (Access & Refresh)
    let (access_token, refresh_token) = generate_tokens(user.id, user.role_id)
        .map_err(|_| "Failed to generate tokens".to_string())?;

    // 6. ส่ง Response กลับไปพร้อม Status 200
    Ok(AuthResponse {
        status: 200, // Login สำเร็จใช้ 200 OK
        message: "Login successful".to_string(),
        access_token,
        refresh_token,
    })
}