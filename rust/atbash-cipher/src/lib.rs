/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let handler = |c: char| -> char { (b'z' - c as u8 + b'a') as char };
    plain
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
        .map(|cs| cs.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let handler = |c: char| -> char { (b'z' + b'a' - c as u8) as char };

    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                handler(c)
            } else {
                c
            }
        })
        .collect::<String>()
}
