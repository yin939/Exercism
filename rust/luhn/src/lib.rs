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
    code.as_str()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, v)| if i % 2 == 0 { v } else { v * 2 })
        .map(|v| if v > 9 { v - 9 } else { v })
        .sum::<u32>() % 10 == 0
}
