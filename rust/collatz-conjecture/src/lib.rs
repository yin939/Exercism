pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut n = n;
    let mut count = 0;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        count += 1;
    }

    Some(count)
}
