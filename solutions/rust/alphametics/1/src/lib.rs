use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Split input into parts and clean up
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side = parts[0].trim();
    let right_side = parts[1].trim();

    // Get left side terms
    let left_terms: Vec<&str> = left_side.split('+').map(|s| s.trim()).collect();

    // Collect all unique letters
    let mut unique_letters: HashSet<char> = HashSet::new();
    let mut first_letters: HashSet<char> = HashSet::new();

    // Add letters from left terms
    for term in &left_terms {
        if let Some(first) = term.chars().next() {
            first_letters.insert(first);
        }
        for c in term.chars() {
            if c.is_alphabetic() {
                unique_letters.insert(c);
            }
        }
    }

    // Add letters from right side
    if let Some(first) = right_side.chars().next() {
        first_letters.insert(first);
    }
    for c in right_side.chars() {
        if c.is_alphabetic() {
            unique_letters.insert(c);
        }
    }

    // If we need more digits than available (0-9), no solution exists
    if unique_letters.len() > 10 {
        return None;
    }

    // Try all possible digit assignments
    let digits: Vec<u8> = (0..=9).collect();
    for perm in digits.iter().permutations(unique_letters.len()) {
        let mut mapping: HashMap<char, u8> = HashMap::new();
        let letters: Vec<char> = unique_letters.iter().cloned().collect();
        
        // Create mapping of letters to digits
        for (letter, &digit) in letters.iter().zip(perm.iter()) {
            mapping.insert(*letter, *digit);
        }

        // Check if any first letter would be zero
        if first_letters.iter().any(|&c| mapping[&c] == 0) {
            continue;
        }

        // Calculate values
        let mut _valid = true;
        let left_sum: u64 = left_terms.iter()
            .map(|term| {
                let mut value = 0u64;
                for c in term.chars() {
                    if c.is_alphabetic() {
                        value = value * 10 + mapping[&c] as u64;
                    }
                }
                value
            })
            .sum();

        let right_value: u64 = {
            let mut value = 0u64;
            for c in right_side.chars() {
                if c.is_alphabetic() {
                    value = value * 10 + mapping[&c] as u64;
                }
            }
            value
        };

        if left_sum == right_value {
            return Some(mapping);
        }
    }

    None
}
