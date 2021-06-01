pub fn raindrops(n: u32) -> String {
    let mut buf = String::new();
    if n % 3 == 0 {
        buf.push_str("Pling");
    }
    if n % 5 == 0 {
        buf.push_str("Plang");
    }
    if n % 7 == 0 {
        buf.push_str("Plong");
    }

    match buf.as_str() {
        "" => n.to_string(),
        _ => buf,
    }
}
