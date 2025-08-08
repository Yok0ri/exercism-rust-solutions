// Rules:
// 1. Discard if length is less than 2
// 2. Discard if non-numeric character is found
// 3. Remove/ignore all white spaces

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c| c.is_numeric() || c.is_whitespace()) {
        return false;
    }

    let digitalized_code: Vec<u32> = code
        .chars()
        .filter(|c| c.is_numeric())
        .filter_map(|n| n.to_digit(10))
        .collect();

    if digitalized_code.len() < 2 {
        return false;
    }

    digitalized_code
        .iter()
        .rev()
        .enumerate()
        .map(|(i, val)| {
            if i % 2 != 0 {
                let double = *val * 2;
                if double > 9 { double - 9 } else { double }
            } else {
                *val
            }
        })
        .sum::<u32>()
        % 10
        == 0
}
