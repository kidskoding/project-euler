use num_bigint::BigUint;
use num_traits::One;

pub fn prob15() -> String {
    (factorial(40) / (factorial(20) * factorial(20))).to_string()
}

fn factorial(n: u32) -> BigUint {
    let mut product = BigUint::one();
    for k in 2..=n {
        product *= k;
    }

    product
}
