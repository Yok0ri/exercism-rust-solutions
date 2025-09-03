// - quotation mark: "Sure."
// - ALL CAPITAL: "Whoa, chill out!"
// - ALL CAPITAL + quotation mark: "Calm down, I know what I'm doing!"
// - silence: "Fine. Be that way!"
// - _ : " Whatever."
pub fn reply(message: &str) -> &str {
    match (
        message.trim().ends_with('?'),
        message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase(),
        message.trim().is_empty(),
    ) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (false, false, false) => "Whatever.",
    }
}
