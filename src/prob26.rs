pub fn prob26() -> usize {
    let mut max_len = 0;
    let mut best = 0;

    for n in (1..1000).rev(){
        let res = 1.0 / (n as f64);
        let decimal_part = res.fract();

        let s = decimal_part.to_string();

        if let Some(decimals) = s.split('.').nth(1) {
            let len = decimals.len();
            if len > max_len {
                max_len = len;
                best = n;
            }
        }
    }

    best
}
