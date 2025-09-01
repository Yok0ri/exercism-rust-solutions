/// Manual check up to `sqrt(x)`
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count: u32 = 1; //2 is already counted
    let mut candidate: u32 = 3;

    loop {
        if is_prime(candidate) {
            if count == n {
                return candidate;
            }
            count += 1;
        }
        candidate += 2; //skipping even numbers after 2
    }
}

fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2..=x.isqrt() {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}
