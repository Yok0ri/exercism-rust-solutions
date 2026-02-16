/// "Sieve of Eratosthenes" method
///
/// Using `bool`
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
fn sieve_of_eratosthenes(upper_n: u64) -> Vec<bool> {
    let mut sieve = vec![true; (upper_n + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;

    let mut p = 2;
    while p * p <= upper_n {
        if sieve[p as usize] {
            let mut multiple = p * p;
            while multiple <= upper_n {
                sieve[multiple as usize] = false;
                multiple += p;
            }
        }
        p += 1;
    }
    sieve
}
fn sieve_decoder(mask: Vec<bool>) -> Vec<u32> {
    mask.into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i as u32) } else { None })
        .collect()
}
