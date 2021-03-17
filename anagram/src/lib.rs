use std::collections::{HashMap, HashSet};

use unicode_segmentation::{self, UnicodeSegmentation};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let word = word.to_lowercase();
  possible_anagrams
    .iter()
    .filter(|&&possible| anagram(&word, &possible.to_lowercase()))
    .map(|&x| x)
    .collect()
}

fn anagram(lhs: &str, rhs: &str) -> bool {
  lhs != rhs && frequencies(lhs) == frequencies(rhs)
}

fn frequencies(word: &str) -> HashMap<&str, i64> {
  word.graphemes(true).fold(HashMap::new(), |mut acc, val| {
    *acc.entry(val).or_insert(0) += 1;
    acc
  })
}
