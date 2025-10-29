pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    let mut number = n;

    while number > 1 {
        if number % divisor == 0 {
            factors.push(divisor);
            number /= divisor;
        } else {
            divisor += 1;
        }
    }

    factors
}
