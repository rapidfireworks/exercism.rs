pub fn brackets_are_balanced(string: &str) -> bool {
  let mut stack: Vec<char> = vec![];
  for bracket in string.chars() {
    match bracket {
      '(' | '{' | '[' => stack.push(bracket),
      ')' | '}' | ']' => match (stack.pop(), bracket) {
        (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => continue,
        _ => return false,
      },
      _ => continue,
    }
  }
  stack.is_empty()
}
