#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    // implement your palindrome type here
    factors: (u64, u64),
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        // unimplemented!("create a palindrome with factors ({}, {})", a, b)
        Palindrome { factors: (a, b) }
    }

    pub fn value(&self) -> u64 {
        self.factors.0 + self.factors.1
    }

    // Actually, I don't understand the meaning of `insert` function.
    pub fn insert(&mut self, a: u64, b: u64) {
        if a * b > self.factors.0 * self.factors.1 {
            self.factors.0 += a;
            self.factors.1 += b;
        }
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }
    let mut min_val: (u64, u64) = (0, 0);
    let mut max_val: (u64, u64) = (0, 0);

    for i in min..=max {
        for j in min..=max {
            if is_palindrome(i * j) {
                if min_val == (0, 0) {
                    if i > j {
                        min_val = (j, i);
                        max_val = (j, i);
                    } else {
                        min_val = (i, j);
                        max_val = (i, j);
                    }
                } else if i * j > max_val.0 * max_val.1 {
                    if i > j {
                        max_val = (j, i);
                    } else {
                        max_val = (i, j);
                    }
                }
            }
        }
    }
    if min_val == (0, 0) || max_val == (0, 0) {
        return None;
    }

    Some((
        Palindrome { factors: min_val },
        Palindrome { factors: max_val },
    ))
}

pub fn is_palindrome(n: u64) -> bool {
    let mut m = n;
    let mut reverse = 0;
    while m > 0 {
        reverse = reverse * 10 + (m % 10);
        m /= 10;
    }

    reverse == n
}
