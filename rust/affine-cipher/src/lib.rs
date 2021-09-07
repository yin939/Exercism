/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let handler = |c: char| -> char {
        let x = c as i32 - 'a' as i32;

        let index = ((a * x + b).rem_euclid(26)) as u8;

        (b'a' + index) as char
    };
    let rel = plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                handler(c)
            } else {
                c
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|f| f.iter().collect::<String>())
        .collect::<Vec<String>>();

    Ok(rel.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, 26) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let handler = |c: char| -> char {
        let y = c as i32 - 'a' as i32;

        let mut a_i = 1;
        while a * a_i % 26 != 1 {
            a_i += 1;
        }

        let index = (a_i * (y - b)).rem_euclid(26) as u8;

        (b'a' + index) as char
    };

    let rel = ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                handler(c)
            } else {
                c
            }
        })
        .collect::<String>();

    Ok(rel)
}

pub fn gcd(m: i32, n: i32) -> i32 {
    if n != 0 {
        gcd(n, m % n)
    } else {
        m
    }
}
