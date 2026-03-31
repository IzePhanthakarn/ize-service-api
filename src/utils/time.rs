use chrono::{DateTime, FixedOffset, Utc};

// 💡 1. สร้าง Constant เก็บ Format ไว้
// %Y = ปี, %m = เดือน, %d = วัน
// %H = ชั่วโมง, %M = นาที, %S = วินาที
// %:z = Timezone Offset แบบมีโคลอน (เช่น +07:00)
pub const STANDARD_DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";

// 2. ฟังก์ชันเดิม: คืนค่าเป็น DateTime Object (เผื่อเอาไปใช้คำนวณ บวก/ลบ วันที่)
pub fn now_thai() -> DateTime<FixedOffset> {
    let thai_offset = FixedOffset::east_opt(7 * 3600).unwrap();
    Utc::now().with_timezone(&thai_offset)
}

// 💡 3. ฟังก์ชันใหม่: คืนค่าเป็น String ที่ถูก Format ตัดเศษวินาทีทิ้งให้เรียบร้อย (พร้อมส่งออก API)
pub fn now_thai_string() -> String {
    now_thai().format(STANDARD_DATETIME_FORMAT).to_string()
}