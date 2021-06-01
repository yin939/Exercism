pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for (index, rows) in input.iter().enumerate() {
        for (col, val) in rows.iter().enumerate() {
            if is_max_in_row(val, rows) && is_min_in_col(val, input, col) {
                points.push((index, col));
            }
        }
    }

    points
}

pub fn is_max_in_row(val: &u64, rows: &Vec<u64>) -> bool {
    rows.iter().all(|x| x <= val)
}

pub fn is_min_in_col(val: &u64, arr: &[Vec<u64>], col: usize) -> bool {
    (0..arr.len()).all(|idx| arr[idx][col] >= *val)
}
