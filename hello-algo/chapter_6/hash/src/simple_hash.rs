/// 加法hsah
pub fn add_hash(key: &str) -> i32 {
    let mut hash = 0_i64;
    const MODULUS: i64 = 1000000007;
    for c in key.chars() {
        hash = (hash + c as i64) % MODULUS;
    }

    hash as i32
}

/// 乘法hsah
pub fn mul_hash(key: &str) -> i32 {
    let mut hash = 0_i64;
    const MODULUS: i64 = 1000000007;
    for c in key.chars() {
        hash = (31 * hash + c as i64) % MODULUS;
    }

    hash as i32
}

/// 异或hsah
pub fn xor_hash(key: &str) -> i32 {
    let mut hash = 0_i64;
    const MODULUS: i64 = 1000000007;
    for c in key.chars() {
        hash ^= c as i64;
    }

    (hash & MODULUS) as i32
}

/// 旋转hsah
pub fn rot_hash(key: &str) -> i32 {
    let mut hash = 0_i64;
    const MODULUS: i64 = 1000000007;
    for c in key.chars() {
        hash = ((hash << 4) ^ (hash >> 28) ^ c as i64) % MODULUS;
    }

    hash as i32
}
