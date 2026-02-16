use crate::prime_numbers_algorithm::primes_until_n;

#[allow(dead_code)]
/// "Sieve of Eratosthenes" method
///
/// Using `Option`
///
/// Public methods:
///
/// - `nth_prime` - returns `n`-th prime number **(u32)**
/// - `primes_until_n` - returns a list of all prime numbers up to `n` **(Vec<u32>)**
mod prime_numbers_algorithm {
    pub fn nth_prime(n: u64) -> u64 {
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
    pub fn primes_until_n(n: u64) -> Vec<u64> {
        sieve_decoder(sieve_of_eratosthenes(n))
    }
    fn sieve_of_eratosthenes(upper_n: u64) -> Vec<Option<u64>> {
        let mut sieve: Vec<Option<u64>> = (2..=upper_n).map(|n| Some(n)).collect();

        let mut p_idx = 0;
        while p_idx < sieve.len() {
            let p = match sieve[p_idx] {
                Some(num) => num,
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
    fn sieve_decoder(mask: Vec<Option<u64>>) -> Vec<u64> {
        mask.into_iter().filter_map(|n| n).collect()
    }
}

/// Pipeline with iterators and adapters
pub fn factors(n: u64) -> Vec<u64> {
    //all non-prime factors must have at least one prime factor ≤ sqrt(n)
    let prime_list = primes_until_n((n as f64).sqrt().ceil() as u64);
    let mut nn = n;

    let mut factors: Vec<u64> = prime_list
        .into_iter()
        .scan(&mut nn, |nn, prime| {
            let mut count = 0;
            while **nn % prime == 0 {
                **nn /= prime;
                count += 1;
            }
            Some(prime_n_times(prime, count))
        })
        .flat_map(|vec| vec)
        .collect();

    //any leftover after factoring all primes ≤ sqrt(n) is guaranteed to be a prime
    if nn > 1 {
        factors.push(nn);
    }

    factors
}

fn prime_n_times(prime: u64, count: i32) -> Vec<u64> {
    vec![prime; count as usize]
}
