use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut vec_in: Vec<char> = word.to_lowercase().chars().collect();
    vec_in.sort();

    possible_anagrams
        .iter()
        .filter(|predicate| {
            let mut temp = predicate.to_lowercase().chars().collect::<Vec<char>>();
            temp.sort();
            temp == vec_in && *predicate.to_lowercase() != word.to_lowercase()
        })
        .copied()
        .collect()
}
