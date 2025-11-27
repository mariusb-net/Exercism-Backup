#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    // compute aliquot sum (sum of proper divisors, excluding the number itself)
    let aliquot_sum = if num == 1 {
        0
    } else {
        let mut sum: u64 = 1;
        let mut k: u64 = 2;
        // iterate up to sqrt(num) safely using u128 to avoid overflow on multiplication
        while (k as u128) * (k as u128) <= num as u128 {
            if num % k == 0 {
                sum += k;
                let other = num / k;
                if other != k {
                    sum += other;
                }
            }
            k += 1;
        }
        sum
    };

    use Classification::*;
    if aliquot_sum == num {
        Some(Perfect)
    } else if aliquot_sum > num {
        Some(Abundant)
    } else {
        Some(Deficient)
    }
}
