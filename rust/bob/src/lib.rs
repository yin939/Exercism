pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    let question = msg.ends_with("?");
    let shouting = msg == msg.to_uppercase() && msg.chars().any(|c| c.is_alphabetic());

    match msg {
        "" => "Fine. Be that way!",
        _ if question && shouting => "Calm down, I know what I'm doing!",
        _ if question => "Sure.",
        _ if shouting => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
