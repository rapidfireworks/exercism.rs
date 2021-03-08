pub fn raindrops(n: u32) -> String {
  let sounds = sounds(&n);
  if sounds.is_empty() {
    n.to_string()
  } else {
    sounds
  }
}

fn sound(n: &u32) -> &str {
  match n {
    3 => "Pling",
    5 => "Plang",
    7 => "Plong",
    _ => "",
  }
}

fn sounds(n: &u32) -> String {
  [3, 5, 7]
    .iter()
    .filter(|&d| n % d == 0)
    .map(sound)
    .collect()
}
