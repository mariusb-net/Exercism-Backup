pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = message.ends_with('?');
    let is_shouting = message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic());
    match (is_question, is_shouting) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
