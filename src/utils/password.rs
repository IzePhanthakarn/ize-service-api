use argon2::{
    password_hash::{ Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString },
    Argon2,
};

use rand_core::OsRng;

// 💡 ฟังก์ชันที่ 1: รับรหัสผ่านปกติมาเข้ารหัส (ใช้ตอน Register)
pub fn hash_password(password: &str) -> Result<String, Error> {
    // 1. สร้าง Salt (เกลือ) คือการสุ่มตัวอักษรมั่วๆ มาผสมกับรหัสผ่านก่อนแฮช
    // Trick: การใส่ Salt ทำให้คนที่ตั้งรหัสผ่านเหมือนกัน (เช่น 123456) จะได้ผลลัพธ์ Hash ไม่เหมือนกัน ป้องกันการเดาแบบตาราง (Rainbow Table)
    let salt = SaltString::generate(&mut OsRng);

    // 2. ดึงการตั้งค่าเริ่มต้นของ Argon2 มาใช้ (ค่า Default ถือว่าปลอดภัยมากแล้วสำหรับปัจจุบัน)
    let argon2 = Argon2::default();

    // 3. ทำการ Hash รหัสผ่าน (ต้องแปลงเป็น byte ด้วย .as_bytes() ก่อน) และผสมกับ Salt
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash) // คืนค่า String ที่ผ่านการ Hash แล้วกลับไป
}

// 💡 ฟังก์ชันที่ 2: ตรวจสอบรหัสผ่าน (ใช้ตอน Login)
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // 1. แปลง String hash ใน Database กลับมาเป็น Object PasswordHash ก่อน
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => {
            return false;
        } // ถ้าแปลงไม่ได้ (เช่น รูปแบบผิด) ให้ตอบกลับไปเลยว่ารหัสผิด (false)
    };

    // 2. เอาหน้าสด (password) มาเทียบกับรหัสที่ Hash ไว้ ว่าตรงกันไหม
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() // ถ้าตรงกัน is_ok() จะคืนค่า true
}
