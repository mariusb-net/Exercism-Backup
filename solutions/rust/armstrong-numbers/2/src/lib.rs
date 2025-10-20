pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let num_digits = digits.len() as u32;
    let sum_of_powers: u32 = digits.iter().map(|&d| d.pow(num_digits)).sum();
    sum_of_powers == num
}
