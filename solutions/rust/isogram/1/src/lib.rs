use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();
    for c in candidate.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()) {
        if !seen.insert(c) {
            return false;
        }
    }
    true
}
