/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() < 2 || !code.chars().all(|c| c.is_digit(10)) {
        return false;
    }
    let sum: u32 = code
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut n = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            n
        })
        .sum();
    sum % 10 == 0
}
