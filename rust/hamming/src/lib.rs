/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut rel = 0_usize;
    let merge = s1.chars().zip(s2.chars()).collect::<Vec<_>>();
    for (c1, c2) in merge.into_iter() {
        if c1 != c2 {
            rel += 1;
        }
    }

    Some(rel)
}
