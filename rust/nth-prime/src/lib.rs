pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut number = 2;
    while count <= n {
        if is_prime(number) {
            count += 1;
        }
        number += 1;
    }
    number - 1
}

fn is_prime(prime: u32) -> bool {
    for i in 2..=((prime as f64).sqrt() as u32) {
        if prime % i == 0 {
            return false;
        }
    }
    true
}
