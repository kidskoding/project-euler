pub fn prob14() -> u128 {
    let mut max_length = 0;
    let mut number_with_max_length = 0;

    for i in (1..1_000_000).rev() {
        let mut n = i;
        let mut count = 1;

        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1;
            }
            count += 1;
        }

        if count > max_length {
            max_length = count;
            number_with_max_length = i;
        }
    }

    number_with_max_length
}
