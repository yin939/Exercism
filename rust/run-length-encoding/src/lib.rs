pub fn encode(source: &str) -> String {
    if source.len() == 0 {
        return "".to_string();
    }
    let mut result = String::new();
    let mut count = 1;
    let mut current = '\0';
    for c in source.chars() {
        if c != current {
            if count != 1 {
                result.push_str(format!("{}{}", count, current).as_str());
            } else if current != '\0' {
                result.push(current);
            }
            current = c;
            count = 1;
        } else {
            count += 1;
        }
    }

    if count != 1 {
        result.push_str(format!("{}{}", count, current).as_str());
    } else {
        result.push(current);
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for c in source.chars() {
        if c.is_ascii_digit() {
            count = count * 10 + c.to_digit(10).unwrap();
            continue;
        }

        if count > 0 && (c.is_ascii_alphabetic() || c.is_ascii_whitespace()) {
            while count > 0 {
                result.push(c);
                count -= 1;
            }
        } else {
            result.push(c);
        }
    }

    result
}
