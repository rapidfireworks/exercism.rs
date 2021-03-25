pub fn nth(n: u32) -> u32 {
  let mut n = n;
  let mut result = 2;
  for offset in [1, 2].iter().chain([2, 4].iter().cycle()) {
    if is_prime(result) {
      if n > 0 {
        n -= 1;
      } else {
        break;
      }
    }
    result += offset;
  }
  result
}

fn is_prime(n: u32) -> bool {
  1 < n && (2..=trunc_sqrt(n)).all(|x| n % x != 0)
}

fn trunc_sqrt(n: u32) -> u32 {
  (n as f64).sqrt() as u32
}
