use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut rel: BTreeMap<char, i32> = BTreeMap::new();
    for (num, chars) in h.iter() {
        for c in chars.iter() {
            rel.insert(c.to_ascii_lowercase(), *num);
        }
    }

    rel
}
