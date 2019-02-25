use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
    let target: Word = word.parse().unwrap();
    possible_anagrams
        .iter()
        .filter(|s| target.is_anagram(&s.parse().unwrap()))
        .cloned()
        .collect()
}

struct Word {
    lowercase: String,
    char_count: HashMap<char, usize>,
}

impl Word {
    fn is_anagram(&self, other: &Word) -> bool {
        self.lowercase != other.lowercase && self.char_count == other.char_count
    }
}

impl FromStr for Word {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();
        let mut char_count = HashMap::new();
        for ch in lowercase.chars() {
            char_count
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        Ok(Self {
            lowercase,
            char_count,
        })
    }
}
