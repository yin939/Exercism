use std::cmp;

use rand::{thread_rng, Rng};

static ASCII_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn encode(key: &str, s: &str) -> Option<String> {
    cipher(key, s, i8::wrapping_add)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    cipher(key, s, i8::wrapping_sub)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let len_key = rng.gen_range(100, cmp::max(s.len(), 200));
    let key: String = (0..len_key)
        .map(|_i| {
            ASCII_LOWER
                .chars()
                .nth(rng.gen_range(0, ASCII_LOWER.len()))
                .unwrap()
        })
        .collect();
    (key.clone(), encode(&key, s).unwrap())
}

pub fn cipher<F>(key: &str, s: &str, op: F) -> Option<String>
where
    F: Fn(i8, i8) -> i8,
{
    if !key.chars().all(|c| c.is_ascii_lowercase()) || key.len() == 0 {
        return None;
    }

    let key_iter = key.bytes().cycle().map(|c| c - b'a');
    Some(
        s.bytes()
            .map(|c| c - b'a')
            .zip(key_iter)
            .map(|(a, b)| (26 + op(a as i8, b as i8)) % 26)
            .map(|c| (c as u8 + b'a') as char)
            .collect(),
    )
}
