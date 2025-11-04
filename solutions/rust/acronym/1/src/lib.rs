pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    // Track whether the next alphabetic character starts a new word.
    // Hyphens and whitespace mark word boundaries; other punctuation is ignored.
    let mut start_of_word = true;
    // Track whether the previous alphabetic character was lowercase to detect
    // camelCase transitions (e.g. 'rT' in HyperText -> include 'T').
    let mut prev_was_lower = false;

    for c in phrase.chars() {
        if c == '-' || c.is_whitespace() {
            // Hyphen and whitespace are word separators.
            start_of_word = true;
            prev_was_lower = false;
            continue;
        }

        if c.is_alphabetic() {
            if start_of_word {
                // First alphabetic char of a word -> include.
                acronym.push(c.to_ascii_uppercase());
                start_of_word = false;
                prev_was_lower = c.is_lowercase();
            } else {
                // Inside a word: include only camelCase uppercase letters
                // where the previous alphabetic char was lowercase.
                if c.is_uppercase() && prev_was_lower {
                    acronym.push(c.to_ascii_uppercase());
                    prev_was_lower = false;
                } else {
                    prev_was_lower = c.is_lowercase();
                }
            }
        } else {
            // Other punctuation (commas, periods, apostrophes, underscores, etc.)
            // are ignored and do not start a new word.
            continue;
        }
    }

    acronym
}
