use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut confirmed_anagrams: HashSet<&str> = HashSet::new();
    let word_small = word.to_lowercase();
    let mut word_vec = word_small.chars().collect::<Vec<_>>();
    word_vec.sort();
    for candidate in possible_anagrams {
        // Two words must be DIFFERENT BEFORE sorting
        let candidate_small = candidate.to_lowercase();
        if word_small == candidate_small {
            continue;
        }

        // Two words must be the SAME AFTER sorting
        let mut candidate_vec = candidate_small.chars().collect::<Vec<_>>();
        candidate_vec.sort();
        if word_vec == candidate_vec {
            confirmed_anagrams.insert(candidate);
        }
    }
    confirmed_anagrams
}
