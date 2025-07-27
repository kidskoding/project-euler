use num_bigint::BigUint;
use num_traits::FromPrimitive;

pub fn prob16() -> String {
    let num = BigUint::from(2 as u32).pow(1000).to_string();
    let mut sum: BigUint = BigUint::ZERO;
    for i in num.chars() {
        if let Some(digit) = i.to_digit(10) {
            sum += BigUint::from_u32(digit).unwrap();
        }
    }

    sum.to_string()
}
