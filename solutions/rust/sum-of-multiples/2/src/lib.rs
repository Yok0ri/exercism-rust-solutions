use std::collections::HashSet;
// - handle case if one of the factors is 0
// - handle case when limit is lower than any given factor

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new();

    for &f in factors {
        if f == 0 {
            continue;
        }

        let mut multiple = f;
        while multiple < limit {
            unique_multiples.insert(multiple);
            multiple += f;
        }
    }

    unique_multiples.iter().sum()
}
