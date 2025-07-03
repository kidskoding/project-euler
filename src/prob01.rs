pub fn prob01() -> u32 {
    let mut sum: u32 = 0;
    for x in 1..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }

    return sum
}