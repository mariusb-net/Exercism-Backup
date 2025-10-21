pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = vec![2];
    let mut num = 3;

    while (primes.len() as u32) <= n {
        let mut is_prime = true;
        for p in &primes {
            if p * p > num {
                break;
            }
            if num % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
        num += 2;
    }
    primes[n as usize]

}
