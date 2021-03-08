pub fn build_proverb(list: &[&str]) -> String {
  if list.is_empty() {
    String::from("")
  } else {
    let end_index = list.len() - 1;
    list
      .iter()
      .zip(list.iter().cycle().skip(1))
      .enumerate()
      .map(|(index, (&curr, &next))| {
        if index < end_index {
          format!("For want of a {} the {} was lost.", curr, next)
        } else {
          format!("And all for the want of a {}.", next)
        }
      })
      .collect::<Vec<String>>()
      .join("\n")
  }
}
