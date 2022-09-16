pub fn big_to_bytes(n: usize) -> [u8; 8] {
    return [(n & (255 << 56)) as u8, (n & (255 << 48)) as u8, (n & (255 << 40)) as u8, (n & (255 << 32)) as u8, (n & (255 << 24)) as u8, (n & (255 << 16)) as u8, (n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_big(bytes: &[u8]) -> usize {
    return ((bytes[0] as usize) << 56) | ((bytes[1] as usize) << 48) | ((bytes[2] as usize) << 40) | ((bytes[3] as usize) << 32) | ((bytes[4] as usize) << 24) | ((bytes[5] as usize) << 16) | ((bytes[6] as usize) << 8) | (bytes[7] as usize);
}

pub fn i64_to_bytes(n: i64) -> [u8; 8] {
    return [(n & (255 << 56)) as u8, (n & (255 << 48)) as u8, (n & (255 << 40)) as u8, (n & (255 << 32)) as u8, (n & (255 << 24)) as u8, (n & (255 << 16)) as u8, (n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_i64(bytes: &[u8]) -> i64 {
    return ((bytes[0] as i64) << 56) | ((bytes[1] as i64) << 48) | ((bytes[2] as i64) << 40) | ((bytes[3] as i64) << 32) | ((bytes[4] as i64) << 24) | ((bytes[5] as i64) << 16) | ((bytes[6] as i64) << 8) | (bytes[7] as i64);
}

pub fn u64_to_bytes(n: u64) -> [u8; 8] {
    return [(n & (255 << 56)) as u8, (n & (255 << 48)) as u8, (n & (255 << 40)) as u8, (n & (255 << 32)) as u8, (n & (255 << 24)) as u8, (n & (255 << 16)) as u8, (n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    return ((bytes[0] as u64) << 56) | ((bytes[1] as u64) << 48) | ((bytes[2] as u64) << 40) | ((bytes[3] as u64) << 32) | ((bytes[4] as u64) << 24) | ((bytes[5] as u64) << 16) | ((bytes[6] as u64) << 8) | (bytes[7] as u64);
}

pub fn i32_to_bytes(n: i32) -> [u8; 4] {
    return [(n & (255 << 24)) as u8, (n & (255 << 16)) as u8, (n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
    return ((bytes[0] as i32) << 24) | ((bytes[1] as i32) << 16) | ((bytes[2] as i32) << 8) | (bytes[3] as i32);
}

pub fn u32_to_bytes(n: u32) -> [u8; 4] {
    return [(n & (255 << 24)) as u8, (n & (255 << 16)) as u8, (n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    return ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
}

pub fn i16_to_bytes(n: i16) -> [u8; 2] {
    return [(n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
    return ((bytes[0] as i16) << 8) | (bytes[1] as i16);
}

pub fn u16_to_bytes(n: u16) -> [u8; 2] {
    return [(n & (255 << 8)) as u8, (n & 255) as u8];
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    return ((bytes[0] as u16) << 8) | (bytes[1] as u16);
}