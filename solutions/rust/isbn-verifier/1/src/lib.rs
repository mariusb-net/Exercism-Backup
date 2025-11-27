/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // Remove hyphens only; other characters remain and will be validated
    let filtered: String = isbn.chars().filter(|&c| c != '-').collect();

    // ISBN-10 must be exactly 10 characters after removing hyphens
    if filtered.len() != 10 {
        return false;
    }

    let mut sum: u32 = 0;

    for (i, c) in filtered.chars().enumerate() {
        let value: u32 = if i == 9 {
            // Check digit: can be 0-9 or 'X' representing 10
            if c == 'X' {
                10
            } else if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                return false;
            }
        } else {
            // Other positions must be digits
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                return false;
            }
        };

        let weight = 10 - i as u32;
        sum += value * weight;
    }

    sum % 11 == 0
}
