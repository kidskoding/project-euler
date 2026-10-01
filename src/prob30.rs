pub fn prob30() -> u32 {
    let mut total_sum = 0;
    let fifth_powers: Vec<u32> = (0..=9).map(|d: u32| d.pow(5)).collect();

    for num in 10..354_294 {
        let sum_of_powers = num
            .to_string()
            .chars()
            .map(|ch| fifth_powers[ch.to_digit(10).unwrap() as usize])
            .sum::<u32>();

        if sum_of_powers == num {
            total_sum += num;
        }
    }

    total_sum
}
