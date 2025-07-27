use num_bigint::BigUint;
use num_traits::One;

pub fn prob20() -> u32 {
    let sum: u32 = factorial(100)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).expect("should be digit!"))
        .sum();

    sum
}

fn factorial(n: u32) -> BigUint {
    let mut product = BigUint::one();
    for k in 2..=n {
        product *= k;
    }

    product
}
