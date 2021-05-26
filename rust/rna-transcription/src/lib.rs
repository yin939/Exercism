#[derive(Debug, PartialEq)]
pub struct Dna {
    dna: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    rna: Vec<char>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut count = 0_usize;
        let mut dnas: Vec<char> = Vec::new();
        for c in dna.chars() {
            if is_dna(c) {
                dnas.push(c);
                count += 1;
            } else {
                return Err(count);
            }
        }

        Ok(Dna { dna: dnas })
    }

    pub fn into_rna(self) -> Rna {
        let mut rna = Vec::new();
        for c in self.dna.into_iter() {
            rna.push(transform(c));
        }

        Rna { rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut rnas = Vec::new();
        let mut count = 0_usize;
        for c in rna.chars() {
            if is_rna(c) {
                rnas.push(c);
                count += 1;
            } else {
                return Err(count);
            }
        }

        Ok(Rna { rna: rnas })
    }
}

fn is_dna(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

fn is_rna(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'U' => true,
        _ => false,
    }
}
fn transform(c: char) -> char {
    match c {
        'A' => 'U',
        'G' => 'C',
        'C' => 'G',
        'T' => 'A',
        _ => '\0',
    }
}
