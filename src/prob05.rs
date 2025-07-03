pub fn prob05() -> u64 {
    let result = 2u64.pow(4) * 3u64.pow(2) * 5 * 7 * 11 * 13 * 17 * 19;
    result
}

pub fn prime_factorization(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    
    while divisor * divisor <= n {
        let mut count = 0;
        while n % divisor == 0 {
            n /= divisor;
            count += 1;
        }
        if count > 0 {
            factors.push((divisor, count));
        }
        divisor += 1;
    }
    
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}
