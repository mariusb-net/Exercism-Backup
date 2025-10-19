use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let target_lower = word.to_lowercase();
    let mut target_chars: Vec<char> = target_lower.chars().collect();
    target_chars.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let cand_lower = candidate.to_lowercase();
            if cand_lower == target_lower {
                return false;
            }
            let mut cand_chars: Vec<char> = cand_lower.chars().collect();
            cand_chars.sort_unstable();
            cand_chars == target_chars
        })
        .copied()
        .collect()
}
