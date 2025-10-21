pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    let first_char = word.chars().next().unwrap();

    // Rule 1: Word begins with a vowel sound.
    if "aeiou".contains(first_char) || word.starts_with("xr") || word.starts_with("yt") {
        return format!("{}ay", word);
    }

    // Rules 2, 3, 4: Word begins with a consonant sound.
    let mut consonant_cluster_len = 0;
    let mut chars = word.chars().peekable();

    while let Some(c) = chars.peek() {
        // 'y' is a vowel if it's not the first letter.
        if "aeiou".contains(*c) || (*c == 'y' && consonant_cluster_len > 0) {
            break;
        }

        // Handle 'qu' as a single unit (Rule 3).
        if *c == 'q' {
            if let Some('u') = chars.clone().nth(1) {
                consonant_cluster_len += 2;
                break;
            }
        }

        consonant_cluster_len += 1;
        chars.next();
    }

    let (consonants, rest) = word.split_at(consonant_cluster_len);
    format!("{}{}ay", rest, consonants)
}
