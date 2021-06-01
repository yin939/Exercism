use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for alphabet in candidate.to_lowercase().chars() {
        if alphabet.is_ascii_alphabetic() {
            if set.contains(&alphabet) {
                return false;
            }
            set.insert(alphabet);
        }
    }

    true
}
