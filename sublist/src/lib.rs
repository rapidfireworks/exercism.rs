#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  use std::cmp::Ordering::*;
  match _first_list.len().cmp(&_second_list.len()) {
    Less if is_sublist(_first_list, _second_list) => Comparison::Sublist,
    Equal if _first_list == _second_list => Comparison::Equal,
    Greater if is_sublist(_second_list, _first_list) => Comparison::Superlist,
    _ => Comparison::Unequal,
  }
}

fn is_sublist<T: PartialEq>(sub: &[T], sup: &[T]) -> bool {
  sub.is_empty() || sup.windows(sub.len()).any(|w| w[0] == sub[0] && w.eq(sub))
}
