pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();
    if len == 0 || len > digits.len() {
        return result;
    }
    let chars: Vec<char> = digits.chars().collect();
    for i in 0..=chars.len() - len {
        let sub: String = chars[i..i + len].iter().collect();
        result.push(sub);
    }
    result
}
