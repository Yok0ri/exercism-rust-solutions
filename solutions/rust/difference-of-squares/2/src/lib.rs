#[allow(dead_code)]
mod v1 {
    pub fn square_of_sum(n: u32) -> u32 {
        (1..=n).sum::<u32>().pow(2)
    }

    pub fn sum_of_squares(n: u32) -> u32 {
        (1..=n).map(|m| m * m).sum()
    }

    pub fn difference(n: u32) -> u32 {
        square_of_sum(n) - sum_of_squares(n)
    }
}

pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (2 * n + 1) * (n + 1) / 6
}

/// Using math formulas:
///
/// **square_of_sum:**   using Gaussian method (add 1 + 2 + ... + n to n + (n-1) + ... + 1), we get `n` terms `n+1`
///
/// **sum_of_squares:**  solving cubic equation for n = 1, n = 2 and n = 3
///
/// Complexity:  `O(1)`, doesn't depend on number of elements
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

// Combinatorial approach:
// square_of_sum:   (sum(i))^2 = sum(i^2) + 2*sum(i*j)
// sum_of_squares:  sum(i^2)
// difference:  sum(i^2) + 2*sum(i*j) - sum(i^2) = 2*sum(i*j), where 1<=i<j<=n
// the result equals twice the sum of all pairwise products of distinct numbers from 1 through n => over all 2-element combinations.
// Complexity:  `O(n^2)`, worse than original `O(n)`
