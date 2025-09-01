/// "Sieve of Eratosthenes" method
///
/// Using `Option`
pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    //calculating upper bound by prime theorem, adding 20% safety net for larger `n`
    let upper_n = if n < 6 {
        15
    } else {
        ((n as f64) * ((n as f64).ln() + (n as f64).ln().ln()) * 1.2).ceil() as u64
    };

    let sieve = sieve_of_eratosthenes(upper_n);
    let primes = sieve_decoder(sieve);
    return primes[n as usize];
}
fn sieve_of_eratosthenes(upper_n: u64) -> Vec<Option<u32>> {
    let mut sieve: Vec<Option<u32>> = (2..=upper_n as u32).map(|n| Some(n)).collect();

    let mut p_idx = 0;
    while p_idx < sieve.len() {
        let p = match sieve[p_idx] {
            Some(num) => num as u64,
            None => {
                p_idx += 1;
                continue;
            }
        };
        if p * p > upper_n {
            break;
        }
        let mut multiple = p * p;
        while multiple <= upper_n {
            sieve[(multiple - 2) as usize] = None;
            multiple += p;
        }
        p_idx += 1;
    }
    sieve
}
fn sieve_decoder(mask: Vec<Option<u32>>) -> Vec<u32> {
    mask.into_iter().filter_map(|n| n).collect()
}
