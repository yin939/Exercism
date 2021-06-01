use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sort = get_sorted(&word_lower);

    possible_anagrams
        .iter()
        .filter(|predicate| {
            let predicate_lower = predicate.to_lowercase();

            word_lower.len() == predicate_lower.len()
                && word_lower != predicate_lower
                && word_sort == get_sorted(&predicate_lower)
        })
        .copied()
        .collect()
}

fn get_sorted(input: &str) -> Vec<char> {
    let mut sorts = input.chars().collect::<Vec<_>>();
    sorts.sort_unstable();
    sorts
}
