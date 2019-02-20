/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transform(plain)
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transform(cipher).iter().collect()
}

fn transform(input: &str) -> Vec<char> {
    input
        .to_lowercase()
        .chars()
        .filter_map(|ch| match ch {
            'a'...'z' => Some((b'z' - ch as u8 + b'a') as char),
            '0'...'9' => Some(ch),
            _ => None,
        })
        .collect()
}
