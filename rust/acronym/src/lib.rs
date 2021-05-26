pub fn abbreviate(phrase: &str) -> String {
    phrase
        // 切割出需要的字符串
        .split(&[' ', '-', '_'][..])
        .filter(|&c| c != "")
        .map(|word| {
            // 用 map 做相关的匹配
            // 匹配 PNG 这种字符串
            if word.to_ascii_uppercase() == word {
                word.get(0..1).unwrap().to_string()
            } else if word.len() > 1 {
                // 主要是匹配 HyperText 这种字符串
                word.char_indices()
                    .filter(|(i, c)| *i == 0 || c.is_ascii_uppercase())
                    .map(|(_, c)| c.to_ascii_uppercase())
                    .collect()
            // 为空的情况
            } else {
                String::from("")
            }
        })
        .collect()
}
