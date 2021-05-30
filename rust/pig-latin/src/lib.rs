pub fn translate(input: &str) -> String {
    let mut rel = String::new();
    for s in input.split_whitespace() {
        rel.push_str(helper(s).as_str());
        rel.push(' ');
    }

    rel.trim_end().to_string()
}

fn helper(input: &str) -> String {
    if input.len() == 2 && input.chars().nth(1).unwrap().eq(&'y') {
        return format!("y{}ay", input.chars().nth(0).unwrap());
    }

    let vowel_sound = ["a", "e", "i", "o", "u", "xr", "yt"];
    let mut rel = String::new();

    if vowel_sound.contains(&input.chars().nth(0).unwrap().to_string().as_str()) || vowel_sound.contains(&&input[0..2]) {
        rel = input.to_string() + "ay";
    } else {
        let mut count = 0;
        for c in input.chars() {
            if !vowel_sound.contains(&c.to_string().as_str()) {
                if count + 1 <= input.len() && input[count..count + 2].eq("qu") {
                    rel = format!("{}{}ay", input[count+2..].to_string(), input[..count+2].to_string());
                    break;
                }
                if count > 0 && input.chars().nth(count).eq(&Some('y')) {
                    rel = format!(
                        "{}{}ay",
                        input[count..].to_string(),
                        input[..count].to_string(),
                    );
                    break;
                }
                count += 1;
            } else {
                rel = input[count..].to_string() + &input[0..count] + "ay";
                break;
            }
        }
    }

    rel
}