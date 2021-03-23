use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
  match _first_list.len().cmp(&_second_list.len()) {
    Ordering::Less if is_sublist(_first_list, _second_list) => Comparison::Sublist,
    Ordering::Equal if _first_list == _second_list => Comparison::Equal,
    Ordering::Greater if is_sublist(_second_list, _first_list) => Comparison::Superlist,
    _ => Comparison::Unequal,
  }
}

fn is_sublist<T: PartialEq>(sub: &[T], sup: &[T]) -> bool {
  (0..=(sup.len() - sub.len())).any(|i| sup[i..i + sub.len()].eq(sub))
}
