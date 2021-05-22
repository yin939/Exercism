pub fn build_proverb(list: &[&str]) -> String {
    if list.len() > 0 {
        let mut result = list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
            .collect::<Vec<_>>();
        result.push(format!("And all for the want of a {}.", list[0]).to_string());
        result.join("\n")
    } else {
        "".to_string()
    }
}
