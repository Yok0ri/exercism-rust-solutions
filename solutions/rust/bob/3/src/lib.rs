// - quotation mark: "Sure."
// - ALL CAPITAL: "Whoa, chill out!"
// - ALL CAPITAL + quotation mark: "Calm down, I know what I'm doing!"
// - silence: "Fine. Be that way!"
// - _ : " Whatever."
mod iteration_1 {
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
}

mod iteration_2 {
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
}

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
