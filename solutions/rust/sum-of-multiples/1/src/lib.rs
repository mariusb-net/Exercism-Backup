pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for n in 0..limit {
        if factors.iter().any(|&factor| factor != 0 && n % factor == 0) {
            sum += n;
        }
    }
    sum
}
