pub fn prob09() -> Option<u32> {
    for m in 1..=500 {
        if 500 % m != 0 {
            continue;
        }

        let n = 500 / m - m;
        if n <= 0 || m <= n {
            continue;
        }

        let a = m * m - n * n;
        let b = 2 * m * n;
        let c = m * m + n * n;

        if a + b + c == 1000 {
            return Some(a * b * c);
        }
    }

    None
}