pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::new(); 1 + digits.len()],
        _ => digits
                .chars()
                .collect::<Vec<_>>()
                .windows(len)
                .map(|window| window.into_iter().collect())
                .collect(),
    }
}
