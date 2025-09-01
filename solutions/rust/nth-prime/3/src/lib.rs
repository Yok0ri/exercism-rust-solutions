/// "Sieve of Eratosthenes" method
///
/// Using `bool`
///
/// Optimized version
pub fn nth(n: u32) -> u32 {
    let mut upper_n = 15;
    // find suitable range dynamically
    loop {
        let sieve = sieve_of_eratosthenes(upper_n);
        let primes = sieve_decoder(sieve);
        if primes.len() > n as usize {
            return primes[n as usize];
        }
        upper_n *= 2; // try again with larger range
    }
}
fn sieve_of_eratosthenes(upper_n: u64) -> Vec<bool> {
    let mut sieve = vec![true; (upper_n / 2) as usize];
    let mut p_idx = 0;

    while p_idx < sieve.len() {
        if sieve[p_idx] {
            let p = 2 * p_idx + 3;
            let mut multiple_idx = ((p * p - 3) / 2) as usize; // only odd numbers
            while multiple_idx < sieve.len() {
                sieve[multiple_idx] = false;
                multiple_idx += p;
            }
        }
        p_idx += 1;
    }
    sieve
}
fn sieve_decoder(mask: Vec<bool>) -> Vec<u32> {
    let mut primes = vec![2];
    primes.extend(mask.into_iter().enumerate().filter_map(|(i, is_prime)| {
        if is_prime {
            Some((2 * i + 3) as u32)
        } else {
            None
        }
    }));
    primes
}
