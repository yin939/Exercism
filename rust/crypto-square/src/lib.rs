pub fn encrypt(input: &str) -> String {
    let s = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<char>>();
    let len = match s.len() {
        0 => return "".to_string(),
        x => x,
    };

    let c = (len as f32).sqrt().ceil() as usize;
    let r = (len as f32).sqrt().floor() as usize;

    let v = s.chunks(c).collect::<Vec<_>>();

    let mut rel = vec!["".to_string(); c];

    for i in 0..c {
        for j in 0..r {
            rel[i].push(*v[j].get(i).unwrap_or(&' '));
        }
    }

    rel.join(" ")
}
