pub fn prob12() -> u128 {
    triangular_divisors_limit(500)
}

fn triangular_divisors_limit(limit: u64) -> u128 {
    let mut n = 1u64;

    loop {
        let (a, b) = if n % 2 == 0 {
            (n / 2, n + 1)
        } else {
            (n, (n + 1) / 2)
        };

        let divisor_count = count_divisors(a) * count_divisors(b);
        if divisor_count > limit {
            return (n * (n + 1) / 2).into();
        }

        n += 1;
    }
}

fn count_divisors(mut n: u64) -> u64 {
    let mut count = 1;
    let mut factor = 2;

    while factor * factor <= n {
        let mut exponent = 0;
        while n % factor == 0 {
            n /= factor;
            exponent += 1;
        }
        count *= exponent + 1;

        if factor == 2 {
            factor += 1;
        } else {
            factor += 2;
        }
    }

    if n > 1 {
        count *= 2;
    }

    count
}
