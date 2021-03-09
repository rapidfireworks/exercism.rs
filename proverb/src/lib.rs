pub fn build_proverb(list: &[&str]) -> String {
  let last = list
    .iter()
    .take(1)
    .map(|&s| format!("And all for the want of a {}.", s));

  list
    .iter()
    .as_slice()
    .windows(2)
    .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
    .chain(last)
    .collect::<Vec<String>>()
    .join("\n")
}
