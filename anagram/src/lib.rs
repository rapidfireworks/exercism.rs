use std::collections::HashSet;

use unicode_segmentation::{self, UnicodeSegmentation};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let word = word.to_lowercase();
  possible_anagrams
    .iter()
    .filter(|&&possible| is_anagram(&word, &possible.to_lowercase()))
    .map(|&x| x)
    .collect()
}

fn is_anagram(lhs: &str, rhs: &str) -> bool {
  if lhs.len() == rhs.len() && lhs != rhs {
    let mut lhs_letters: Vec<_> = lhs.graphemes(true).collect();
    lhs_letters.sort();
    let mut rhs_letters: Vec<_> = rhs.graphemes(true).collect();
    rhs_letters.sort();
    lhs_letters == rhs_letters
  } else {
    false
  }
}
