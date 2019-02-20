pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for ch in string.chars() {
        match ch {
            '[' | '{' | '(' => stack.push(ch),
            ']' | '}' | ')' => {
                if let Some(open) = stack.pop() {
                    match (open, ch) {
                        ('[', ']') | ('{', '}') | ('(', ')') => (),
                        _ => return false,
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
