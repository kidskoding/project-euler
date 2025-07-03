pub fn prob06() -> i128 {
    let a: i128 = 100 * 101 / 2;
    let sum_squared = a.pow(2);
    let squared_sum = (100 * 101 * (2 * 100 + 1)) / 6;
    sum_squared - squared_sum
}