pub fn square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

/// Sn = 1 + 2 + 4 + 8 + ...
/// b1 = 1, q = 2
/// Sn = b1 * (1 -q^n) / (1 - q)
/// Sn = 1 * (1 - 2^n) / (1 - 2) = 2^n - 1
/// u64 can hold a 64bit - 1 max value, so exactly 2^n - 1
pub fn total() -> u64 {
    u64::MAX
}
