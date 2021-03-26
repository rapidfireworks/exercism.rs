pub fn brackets_are_balanced(string: &str) -> bool {
  let mut stack: Vec<char> = vec![];
  for bracket in string.chars() {
    match bracket {
      '(' | '{' | '[' => stack.push(bracket),
      ')' | '}' | ']' => {
        let popped = stack.pop();
        if popped.is_none() || popped != opening_pair(bracket) {
          return false;
        }
      }
      _ => (),
    }
  }
  stack.is_empty()
}

fn opening_pair(bracket: char) -> Option<char> {
  match bracket {
    ')' => Some('('),
    '}' => Some('{'),
    ']' => Some('['),
    _ => None,
  }
}
