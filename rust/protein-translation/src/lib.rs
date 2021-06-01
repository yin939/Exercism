use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.pairs.get(&codon) {
            Some(v) => Some(*v),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        // 可以去掉这个判断
        if rna.len() < 3 {
            return None;
        }
        let stops = ["UAA", "UAG", "UGA"];
        let mut rel = Vec::new();
        let mut len = rna.len();

        for i in 0..(rna.len() / 3) {
            let start = i * 3;
            let temp = &rna[start..start + 3];
            len -= 3;
            if stops.contains(&temp) {
                break;
            }
            // make sure the correct length
            if len < 3 && len > 0 {
                return None;
            }

            match self.pairs.get(temp) {
                Some(&s) => rel.push(s),
                None => break,
            }
        }

        if rel.is_empty() {
            None
        } else {
            Some(rel)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut rel = HashMap::new();
    for (c, n) in pairs.into_iter() {
        rel.insert(c, n);
    }
    CodonsInfo { pairs: rel }
}
