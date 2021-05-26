use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let mut sum: usize = 0;
    for c in dna.chars() {
        if is_nucleotide(c) {
            if nucleotide == c {
                sum += 1;
            }
        } else {
            return Err(c);
        }
    }

    Ok(sum)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotide_map: HashMap<char, usize> = HashMap::new();
    for c in "ACGT".chars() {
        nucleotide_map.entry(c).or_default();
    }
    for c in dna.chars() {
        if is_nucleotide(c) {
            let count = nucleotide_map.entry(c).or_insert(0);
            *count += 1;
        } else {
            return Err(c);
        }
    }

    Ok(nucleotide_map)
}

pub fn is_nucleotide(c: char) -> bool {
    match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}
