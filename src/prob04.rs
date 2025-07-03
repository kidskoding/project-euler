pub fn prob04() -> u128 {
    let mut max_palindrome: u128 = 1;
    for i in (99..=999).rev() {
        for j in (100..=i).rev() {
            let product: u128 = i * j;
            if is_palindrome(&product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }

    max_palindrome
}

pub fn is_palindrome(n: &u128) -> bool {
    let chars: Vec<char> = n.to_string().chars().collect();
    for k in 0..chars.len() / 2 {
        if chars[k] != chars[chars.len() - 1 - k] {
            return false;
        }
    }

    true
}