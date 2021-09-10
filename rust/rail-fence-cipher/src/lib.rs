pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rel = vec!["".to_string(); self.rails as usize];
        let mut index = 0_usize;
        let mut down = true;
        for c in text.chars() {
            rel[index].push(c);
            if down {
                index += 1;
                if index == self.rails {
                    index -= 2;
                    down = !down;
                }
            } else {
                if index == 0 {
                    index += 1;
                    down = !down;
                } else {
                    index -= 1;
                }
            }
        }

        rel.concat()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut it = cipher.chars();
        let mut v = vec![' '; cipher.len()];
        let width = 2 * self.rails - 2;
        
        for r in 0..self.rails {
            let mut idx1: usize = r;
            let mut idx2: usize = width - r;
            let skip: bool = idx1 == idx2 || idx2 == width;
            loop {
                if idx1 >= v.len() {
                    break;
                }
                v[idx1] = it.next().unwrap();
                idx1 += width;
                if !skip {
                    if idx2 >= v.len() {
                        break;
                    }
                    v[idx2] = it.next().unwrap();
                    idx2 += width;
                }
            }
        }
        v.iter().collect()
    }
}
