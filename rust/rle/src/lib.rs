use std::iter::repeat;

pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut prev_ch: char;
    let mut count = 1;

    if let Some(ch) = source.chars().nth(0) {
        prev_ch = ch;
    } else {
        return encoded;
    }

    for ch in source.chars().skip(1) {
        if ch == prev_ch {
            count += 1;
        } else {
            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(prev_ch);
            prev_ch = ch;
            count = 1;
        }
    }
    if count > 1 {
        encoded.push_str(&count.to_string());
    }
    encoded.push(prev_ch);
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut number = String::new();
    let mut count;

    for ch in source.chars() {
        if ch.is_digit(10) {
            number.push(ch);
        } else {
            count = number.parse().unwrap_or(1);
            decoded.push_str(&repeat(ch).take(count).collect::<String>());
            number.clear();
        }
    }
    decoded
}
