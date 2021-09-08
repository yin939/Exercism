pub fn get_diamond(c: char) -> Vec<String> {
    let length = ((c as u8 - b'A' + 1) * 2 - 1) as usize;
    let mut rel = vec![vec![' '; length]; length];
    let center = length / 2;

    for (i, item) in rel.iter_mut().enumerate() {
        let left = (center as isize - i as isize).abs() as usize;
        let right = length - 1 - left;
        let c = ((length - i - 1).min(i) as u8 + b'A') as char;
        item[left] = c;
        item[right] = c;
    }

    rel.into_iter()
        .map(|f| f.iter().collect::<String>())
        .collect::<Vec<String>>()
}
