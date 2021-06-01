pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();
    let mut factor = 2;
    while n > 1 {
        if n % factor == 0 {
            factors.push(factor);
            n /= factor;
        } else {
            factor += 1;
        }
    }

    factors
}
