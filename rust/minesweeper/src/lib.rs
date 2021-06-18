use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let len_row = minefield.len();
    let len_col = minefield[0].len();

    let mut rel = Vec::new();

    // closure
    let is_mine = |row: usize, col: usize| minefield[row].as_bytes()[col] == b'*';
    let count = |row: usize, col: usize| {
        let mut counts = 0;

        // treat the boundary with Saturating arithmetic
        for i in row.saturating_sub(1)..min(row + 2, len_row) {
            for j in col.saturating_sub(1)..min(col + 2, len_col) {
                if (i != row || j != col) && is_mine(i, j) {
                    counts += 1;
                }
            }
        }

        counts
    };

    for i in 0..len_row {
        let mut temp = String::with_capacity(len_col);
        for j in 0..len_col {
            let c = if is_mine(i, j) {
                '*'
            } else {
                match count(i, j) {
                    0 => ' ',
                    n => (n + b'0') as char,
                }
            };
            temp.push(c);
        }
        rel.push(temp);
    }

    rel
}
