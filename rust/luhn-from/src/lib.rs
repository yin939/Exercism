pub struct Luhn {
    valid: bool,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            valid: is_valid(&input.to_string()),
        }
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code
        .trim()
        .split_whitespace()
        .flat_map(|s| s.chars())
        .collect::<String>();

    if code.len() <= 1 || !code.as_str().chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    code.chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, v)| if i % 2 == 0 { v } else { v * 2 })
        .map(|v| if v > 9 { v - 9 } else { v })
        .sum::<u32>()
        % 10
        == 0
}
