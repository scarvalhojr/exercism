use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_alphabetic())
        .all(|ch| chars.insert(ch))
}
