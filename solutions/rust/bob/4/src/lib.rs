// - quotation mark: "Sure."
// - ALL CAPITAL: "Whoa, chill out!"
// - ALL CAPITAL + quotation mark: "Calm down, I know what I'm doing!"
// - silence: "Fine. Be that way!"
// - _ : " Whatever."
pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if is_silence(m) => "Fine. Be that way!",
        m if is_question(m) && is_shouting(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_shouting(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_shouting(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic())
}

fn is_silence(message: &str) -> bool {
    message.is_empty()
}
