pub fn prob03() -> u128 {
    let mut n = 600851475143 as u128;
    let mut factor = 2 as u128;

    while n % factor == 0 {
        n /= factor;
    }

    factor = 3;

    while factor * factor <= n {
        if n % factor == 0 {
            n /= factor;
        } else {
            factor += 2;
        }
    }

    n
}
