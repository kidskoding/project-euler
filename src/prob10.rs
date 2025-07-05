pub fn prob10() -> u128 {
    let mut sum: u128 = 0;
    
    for k in (2..2_000_000).rev() {
        if is_prime(k) {
            sum += k;
        }
    }

    sum
}

fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }

    let limit = (n as f64).sqrt() as u128 + 1;
    for i in 2..limit {
        if n % i == 0 { return false; }
    }
    
    true
}