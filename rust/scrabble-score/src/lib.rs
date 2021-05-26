/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut sum = 0;
    for c in word.to_ascii_uppercase().chars() {
        println!("{}", c);
        sum += find_score(c);
    }

    sum
}

pub fn find_score(c: char) -> u64 {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}
