pub fn rotate(input: &str, key: i8) -> String {
    let mut rel = String::new();
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let mut temp = if c.is_uppercase() {
                (c as u8 - b'A') as i8 + key
            } else {
                (c as u8 - b'a') as i8 + key
            };
            temp %= 26;
            let temp = if temp < 0 { temp + 26 } else { temp };
            let temp = if c.is_uppercase() {
                (temp as u8 + b'A') as char
            } else {
                (temp as u8 + b'a') as char
            };
            rel.push(temp);
        } else {
            rel.push(c);
        }
    }
    rel
}
