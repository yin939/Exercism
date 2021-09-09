pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
}

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
