use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut confirmed_anagrams: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());

    let word_small = word.to_lowercase();
    let word_small_sorted_vec = to_sorted_vec(&word_small);

    for candidate in possible_anagrams {
        // Two words must be DIFFERENT BEFORE sorting
        let candidate_small = candidate.to_lowercase();
        if word_small == candidate_small {
            continue;
        }

        // Two words must be the SAME AFTER sorting
        let candidate_small_vec_sorted = to_sorted_vec(&candidate_small);
        if word_small_sorted_vec == candidate_small_vec_sorted {
            confirmed_anagrams.insert(candidate);
        }
    }
    confirmed_anagrams
}

fn to_sorted_vec(string: &str) -> Vec<char> {
    let mut word_vec = string.chars().collect::<Vec<_>>();
    word_vec.sort();
    word_vec
}
