use std::collections::HashMap;

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    if lines.is_empty() || lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    if lines[0].is_empty() || lines[0].len() % 3 != 0 {
        return Err(Error::InvalidColumnCount(lines[0].len()));
    }

    let row_count = lines.len() / 4;
    let col_count = lines[0].len() / 3;
    let trans_table: HashMap<String, char> = vec![
        ("     |  |   ".to_owned(), '1'),
        (" _  _||_    ".to_owned(), '2'),
        (" _  _| _|   ".to_owned(), '3'),
        ("   |_|  |   ".to_owned(), '4'),
        (" _ |_  _|   ".to_owned(), '5'),
        (" _ |_ |_|   ".to_owned(), '6'),
        (" _   |  |   ".to_owned(), '7'),
        (" _ |_||_|   ".to_owned(), '8'),
        (" _ |_| _|   ".to_owned(), '9'),
        (" _ | ||_|   ".to_owned(), '0'),
    ]
    .into_iter()
    .collect();

    let mut parts = vec![];

    for r in 0..row_count {
        let mut part = String::new();
        for c in 0..col_count {
            let mut digit_raw = String::new();
            for i in 0..4 {
                for j in 0..3 {
                    digit_raw.push(lines[r * 4 + i][c * 3 + j]);
                }
            }
            let digit = match trans_table.get(&digit_raw) {
                None => '?',
                Some(&d) => d,
            };
            part.push(digit);
        }
        parts.push(part);
    }
    Ok(parts.join(","))
}
