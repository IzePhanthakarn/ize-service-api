use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

// 💡 Struct สำหรับเก็บข้อมูลลงไปใน Token (เรียกว่า Claims)
// เราเก็บแค่ข้อมูลที่จำเป็น ห้ามเก็บ Password เด็ดขาด!
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,      // Subject: เก็บ User ID
    pub role_id: Uuid,  // เก็บ Role ID เพื่อเอาไปเช็คสิทธิ์ (SA, Admin, User)
    pub exp: usize,     // Expiration: เวลาหมดอายุ (เป็น Timestamp แบบตัวเลข)
    pub iat: usize,     // Issued At: เวลาที่ออก Token
}

// 💡 ฟังก์ชันสร้าง Token แบบคู่ (คืนค่าเป็น Tuple: Access Token, Refresh Token)
pub fn generate_tokens(user_id: Uuid, role_id: Uuid) -> Result<(String, String), jsonwebtoken::errors::Error> {
    // 1. ดึง Secret Key จากไฟล์ .env (กุญแจสำหรับเซ็นรับรอง Token)
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    // 2. หาเวลาปัจจุบัน
    let now = Utc::now();

    // 3. สร้าง Access Token (กำหนดอายุ 1 ชม.ตาม Best Practice)
    let access_expiration = now + Duration::hours(1);
    let access_claims = Claims {
        sub: user_id,
        role_id,
        exp: access_expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    // ทำการเข้ารหัสด้วยอัลกอริทึม HS256 (Default ของไลบรารี)
    let access_token = encode(&Header::default(), &access_claims, &encoding_key)?;

    // 4. สร้าง Refresh Token (กำหนดอายุ 7 วัน)
    let refresh_expiration = now + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id,
        role_id,
        exp: refresh_expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)?;

    // 5. ส่งคืนทั้งสองตัวกลับไป
    Ok((access_token, refresh_token))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    
    // ใช้การตั้งค่า Default (อัลกอริทึม HS256 และเช็ควันหมดอายุให้ทันที)
    let validation = Validation::default(); 

    // ถอดรหัส ถ้าสำเร็จจะคืนค่า Claims ที่เราแพ็คไว้กลับมา
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    
    Ok(token_data.claims)
}