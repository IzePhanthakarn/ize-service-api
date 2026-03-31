use crate::config::database::DbPool;
// 💡 เปลี่ยนมาเรียกใช้ RegisterResponse และ UserResponse
use crate::modules::auth::dtos::{
    AuthResponse,
    LoginRequest,
    RegisterRequest,
    RegisterResponse,
    UserResponse,
};
use crate::modules::users::models::User;
use crate::modules::users::{ models::NewUser, repository as user_repo };
use crate::utils::id_gen::gen_user_id;
use crate::utils::jwt::generate_tokens;
use crate::utils::password::{ hash_password, verify_password };
use crate::utils::time::STANDARD_DATETIME_FORMAT; // 💡 ดึง Format เวลาของเรามาใช้
use diesel::Connection;

pub async fn register(pool: &DbPool, req: RegisterRequest) -> Result<RegisterResponse, String> {
    let mut conn = pool.get().map_err(|_| "Database connection error".to_string())?;

    let hashed_password = hash_password(&req.password).map_err(|_|
        "Failed to hash password".to_string()
    )?;

    // 💡 เริ่มต้น Transaction
    let created_user = conn
        .transaction::<User, diesel::result::Error, _>(|conn| {
            let new_user = NewUser {
                id: gen_user_id(),
                email: req.email.clone(),
                password_hash: Some(hashed_password),
                role_id: "ro_user".to_string(),
                google_id: None,
            };

            // 1. Insert User
            let user = user_repo::create_user(conn, &new_user)?; // 💡 ใช้ ? ได้เลย เพราะ Type ตรงกันแล้ว

            // 2. Insert Detail
            user_repo::create_user_detail(conn, &user.id)?; // 💡 ใช้ ? ได้เลย

            Ok(user)
        })
        .map_err(|e| {
            // 💡 มาจัดการแปลง Error เป็น String ตรงนี้แทน (ข้างนอก transaction)
            eprintln!("Registration transaction error: {:?}", e);
            "Registration failed: Email already exists or internal error".to_string()
        })?;

    // 💡 เมื่อ Transaction สำเร็จ (Commit แล้ว) ถึงจะส่ง Response กลับ
    Ok(RegisterResponse {
        status: 201,
        message: "User registered successfully".to_string(),
        user: UserResponse {
            id: created_user.id,
            email: created_user.email,
            role_id: created_user.role_id,
            created_at: created_user.created_at.format(STANDARD_DATETIME_FORMAT).to_string(),
        },
    })
}

pub async fn login(pool: &DbPool, req: LoginRequest) -> Result<AuthResponse, String> {
    let mut conn = pool.get().map_err(|_| "Database connection error".to_string())?;

    // 1. ค้นหา User จาก Email
    let user = match user_repo::get_user_by_email(&mut conn, &req.email) {
        Ok(u) => u,
        // Trick: แม้จะหาอีเมลไม่เจอ เราก็ควรตอบกว้างๆ ว่า "Invalid email or password"
        // เพื่อป้องกันไม่ให้ Hacker รู้ว่ามีอีเมลนี้อยู่ในระบบหรือเปล่า (Security Best Practice)
        Err(_) => {
            return Err("Invalid email or password".to_string());
        }
    };

    // 2. ตรวจสอบว่า User มีรหัสผ่านไหม (อาจจะ Login ผ่าน Google OAuth มาก่อนเลยไม่มีรหัส)
    let password_hash = match user.password_hash {
        Some(hash) => hash,
        None => {
            return Err("Please login using your OAuth provider".to_string());
        }
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
    let (access_token, refresh_token) = generate_tokens(user.id, user.role_id).map_err(|_|
        "Failed to generate tokens".to_string()
    )?;

    // 6. ส่ง Response กลับไปพร้อม Status 200
    Ok(AuthResponse {
        status: 200, // Login สำเร็จใช้ 200 OK
        message: "Login successful".to_string(),
        access_token,
        refresh_token,
    })
}
