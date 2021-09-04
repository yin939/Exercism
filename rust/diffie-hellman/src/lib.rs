use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    fast_power_modulus_algorithm(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    fast_power_modulus_algorithm(b_pub, a, p)
}

/// 快速幂取模
pub fn fast_power_modulus_algorithm(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut exp = exp;
    let mut base = base;
    let mut rel = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            rel = rel * base % modulus;
        }
        base = base * base % modulus;
        exp >>= 1;
    }

    rel
}
