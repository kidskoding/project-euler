pub fn prob07() -> u128 {
    let (mut i, mut counter) = (2u128, 0u128);
    while counter < 10001 {
        if is_prime(i) { counter += 1; }
        i += 1
    }

    i - 1
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