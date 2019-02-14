use std::iter::repeat;

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 0;

    while let Some(ch) = chars.next() {
        count += 1;
        if chars.peek() != Some(&ch) {
            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(ch);
            count = 0;
        }
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut digits = String::new();

    for ch in source.chars() {
        if ch.is_digit(10) {
            digits.push(ch);
        } else {
            decoded.extend(repeat(ch).take(digits.parse().unwrap_or(1)));
            digits.clear();
        }
    }
    decoded
}
