extern crate regex;
use regex::Regex;
use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words = words.to_lowercase();
    let re = Regex::new(r"\w+'?\w|\d+").unwrap();
    let mut rel = HashMap::<String, u32>::new();

    for (_, c) in re.captures_iter(&words).enumerate() {
        *rel.entry((&c[0]).to_string()).or_default() += 1;
    }

    rel
}
