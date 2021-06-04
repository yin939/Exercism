pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    let command = command.strip_prefix("What is ")?.strip_suffix("?")?;
    let mut iter = command.split_whitespace();
    let mut cur = iter.next()?.parse::<i32>().ok()?;
    while let Some(item) = iter.next() {
        match item {
            "plus" => {
                let next = iter.next()?.parse::<i32>().ok()?;
                cur += next;
            }
            "minus" => {
                let next = iter.next()?.parse::<i32>().ok()?;
                cur -= next;
            }
            "multiplied" => {
                if iter.next()? != "by" {
                    return None;
                }
                let next = iter.next()?.parse::<i32>().ok()?;
                cur *= next;
            }
            "divided" => {
                if iter.next()? != "by" {
                    return None;
                }
                let next = iter.next()?.parse::<i32>().ok()?;
                cur /= next;
            }
            _ => return None,
        }
    }

    Some(cur)
}
