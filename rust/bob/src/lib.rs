pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let question = trimmed.ends_with('?');
    let shout = !trimmed.chars().any(|ch| ch.is_lowercase())
        && trimmed.chars().any(|ch| ch.is_uppercase());

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if question && shout {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if shout {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
