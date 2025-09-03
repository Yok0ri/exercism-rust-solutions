// - quotation mark: "Sure."
// - ALL CAPITAL: "Whoa, chill out!"
// - ALL CAPITAL + quotation mark: "Calm down, I know what I'm doing!"
// - silence: "Fine. Be that way!"
// - _ : " Whatever."
pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    let has_letters = trimmed.chars().any(|c| c.is_alphabetic());
    let is_question = trimmed.ends_with('?');
    let is_shouting = has_letters && trimmed == trimmed.to_uppercase();
    let is_silence = trimmed.is_empty();

    match (is_question, is_shouting, is_silence) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (false, false, false) => "Whatever.",
    }
}
