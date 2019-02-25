use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    candidates: &[&'a str],
) -> HashSet<&'a str> {
    let target = word.to_lowercase();
    let sorted_chars = sort_chars(&target);

    candidates
        .iter()
        .cloned()
        .filter(|s| {
            let candidate = s.to_lowercase();
            candidate != target && sort_chars(&candidate) == sorted_chars
        })
        .collect()
}

fn sort_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<_> = word.chars().collect();
    chars.sort_unstable();
    chars
}
