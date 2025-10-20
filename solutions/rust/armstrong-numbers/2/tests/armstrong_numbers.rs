use armstrong_numbers::*;

#[test]
fn zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
// #[Ignore]
fn single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
// #[Ignore]
fn there_are_no_two_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
// #[Ignore]
fn three_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
// #[Ignore]
fn three_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
// #[Ignore]
fn four_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_474))
}

#[test]
// #[Ignore]
fn four_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_475))
}

#[test]
// #[Ignore]
fn seven_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
// #[Ignore]
fn seven_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_926_314))
}
