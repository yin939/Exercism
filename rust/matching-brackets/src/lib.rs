pub fn brackets_are_balanced(string: &str) -> bool {
    let mut heap: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' => heap.push(')'),
            '[' => heap.push(']'),
            '{' => heap.push('}'),
            ')' | ']' | '}' => {
                if heap.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }

    heap.is_empty()
}
