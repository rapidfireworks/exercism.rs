use regex::Regex;

pub fn reply(message: &str) -> &str {
  let message = message.trim();
  if message.is_empty() {
    "Fine. Be that way!"
  } else {
    match (
      asking(message).unwrap_or_default(),
      yelling(message).unwrap_or_default(),
    ) {
      (true, true) => "Calm down, I know what I'm doing!",
      (true, false) => "Sure.",
      (false, true) => "Whoa, chill out!",
      (false, false) => "Whatever.",
    }
  }
}

fn asking(message: &str) -> Result<bool, regex::Error> {
  let re = Regex::new(r"\?$")?;
  let capture = re.captures_iter(message).next();
  Ok(capture.is_some())
}

fn yelling(message: &str) -> Result<bool, regex::Error> {
  let letter = Regex::new(r"\p{L}")?;
  let lower = Regex::new(r"\p{Ll}")?;
  let has_letter = letter.captures_iter(message).next();
  let has_lower = lower.captures_iter(message).next();
  Ok(has_letter.is_some() && has_lower.is_none())
}
