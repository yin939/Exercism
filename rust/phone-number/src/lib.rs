pub fn number(user_number: &str) -> Option<String> {
    let mut number = user_number
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    if number.len() < 10 || number.len() > 11 {
        return None;
    }

    if number.len() == 11 {
        if number[0] == 1 {
            number.remove(0);
        } else {
            return None;
        }
    }

    if number[0] < 2 || number[3] < 2 {
        return None;
    }

    Some(number.iter().map(|x| x.to_string()).collect())
}
