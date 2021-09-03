// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::<&str, i8>::new();
    for word in magazine.iter() {
        *words.entry(word).or_default() += 1;
    }

    for word in note.iter() {
        let temp = words.entry(word).or_default();
        if *temp == 0 {
            return false;
        }
        *temp -= 1;
    }

    true
}
