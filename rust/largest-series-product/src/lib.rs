#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if string_digits.chars().any(|f| !f.is_ascii_digit()) {
        return Err(Error::InvalidDigit(
            string_digits
                .chars()
                .filter(|predicate| !predicate.is_ascii_digit())
                .take(1)
                .next()
                .unwrap(),
        ));
    }
    let a = string_digits
        .chars()
        .map(|f| f.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    Ok(a.windows(span).map(|f| f.iter().product()).max().unwrap())
}
