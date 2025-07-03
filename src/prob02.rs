pub fn prob02() -> u128 {
    let mut sum: u128 = 0;
    let (mut a, mut b) = (1 as u128, 2 as u128);

    while b < 4_000_000 {
        if b % 2 == 0 {
            sum += b;
        }

        let next = a + b;
        a = b;
        b = next;
    }

    return sum;
}