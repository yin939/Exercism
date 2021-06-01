use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut alphabet_set = HashSet::new();

    for c in sentence.to_ascii_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            alphabet_set.insert(c);
        }
    }

    alphabet_set.len() == 26
}
