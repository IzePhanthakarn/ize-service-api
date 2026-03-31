use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

// 💡 Struct สำหรับเก็บข้อมูลลงไปใน Token (เรียกว่า Claims)
// เราเก็บแค่ข้อมูลที่จำเป็น ห้ามเก็บ Password เด็ดขาด!
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject: เก็บ User ID
    pub role_id: String,  // เก็บ Role ID เพื่อเอาไปเช็คสิทธิ์ (SA, Admin, User)
    pub exp: usize,     // Expiration: เวลาหมดอายุ (เป็น Timestamp แบบตัวเลข)
    pub iat: usize,     // Issued At: เวลาที่ออก Token
}

// 💡 ฟังก์ชันสร้าง Token แบบคู่ (คืนค่าเป็น Tuple: Access Token, Refresh Token)
pub fn generate_tokens(user_id: String, role_id: String) -> Result<(String, String), jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    let now = Utc::now();

    // 💡 3. สร้าง Access Token
    let access_expiration = now + Duration::hours(1);
    let access_claims = Claims {
        sub: user_id.clone(),
        role_id: role_id.clone(),
        exp: access_expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let access_token = encode(&Header::default(), &access_claims, &encoding_key)?;

    // 💡 4. สร้าง Refresh Token
    let refresh_expiration = now + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id, 
        role_id,
        exp: refresh_expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)?;

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