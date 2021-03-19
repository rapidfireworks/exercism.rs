use std::collections::HashSet;

use unicode_segmentation::{self, UnicodeSegmentation};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let word = word.to_lowercase();
  let word_len = word.len();
  let word_checksum = checksum(&word);
  possible_anagrams
    .iter()
    .filter(|&&possible| {
      let possible = possible.to_lowercase();
      word_len == possible.len()
        && word_checksum == checksum(&possible)
        && is_anagram(&word, &possible)
    })
    .map(|&x| x)
    .collect()
}

fn is_anagram(lhs: &str, rhs: &str) -> bool {
  if lhs != rhs {
    let mut lhs_letters: Vec<_> = lhs.graphemes(true).collect();
    lhs_letters.sort();
    let mut rhs_letters: Vec<_> = rhs.graphemes(true).collect();
    rhs_letters.sort();
    lhs_letters == rhs_letters
  } else {
    false
  }
}

fn checksum(word: &str) -> u8 {
  word.bytes().fold(0, |acc, val| acc.wrapping_add(val))
}
