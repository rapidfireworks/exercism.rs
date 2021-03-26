pub fn reply(message: &str) -> &str {
  let message = message.trim_end();
  if message.is_empty() {
    "Fine. Be that way!"
  } else {
    match (message.ends_with("?"), yelling(message)) {
      (true, true) => "Calm down, I know what I'm doing!",
      (true, false) => "Sure.",
      (false, true) => "Whoa, chill out!",
      (false, false) => "Whatever.",
    }
  }
}

fn yelling(message: &str) -> bool {
  message.chars().any(|letter| letter.is_alphabetic())
    && !message.chars().any(|letter| letter.is_lowercase())
}
