use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut rel: HashSet<[u32; 3]> = HashSet::new();

    for a in 3..(sum / 3) {
        for b in (a + 1)..(sum / 2) {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                rel.insert([a, b, c]);
            }
        }
    }

    rel
}
