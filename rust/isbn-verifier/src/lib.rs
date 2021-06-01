/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // define here to ISBN-10 or ISBN-13 or else...
    let needed = 10_usize;
    let modified_isbn = isbn
        .chars()
        .filter(|x| x.is_ascii_digit() || *x == 'X')
        .collect::<Vec<_>>();
    let len = modified_isbn.len();

    if len != needed {
        return false;
    }
    let mut sum: usize = 0;
    for (i, c) in modified_isbn.into_iter().enumerate() {
        let num = match c {
            'X' if i < len - 1 => return false,
            'X' => 10,
            c => c.to_digit(10).unwrap(),
        };

        sum += (needed - i) * num as usize;
    }

    sum % (len + 1) == 0
}
