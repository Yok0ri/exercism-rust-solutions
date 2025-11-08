pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut k = n;
    let mut i = 0;
    while k > 1 {
        i = i + 1;
        if k % 2 == 0 {
            k = k / 2;
        } else {
            k = 3 * k + 1;
        }
    }

    return Some(i);
}
