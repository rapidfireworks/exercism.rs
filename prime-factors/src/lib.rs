pub fn factors(n: u64) -> Vec<u64> {
  let mut n = n;
  let mut factor = 0;
  let mut result = Vec::<u64>::new();
  for offset in [2, 1, 2].iter().chain([2, 4].iter().cycle()) {
    if n < 2 {
      break;
    }

    factor += offset;
    if !is_prime(factor) {
      continue;
    }

    while n % factor == 0 {
      n /= factor;
      result.push(factor);
    }
  }
  result
}

fn is_prime(n: u64) -> bool {
  1 < n && (2..=trunc_sqrt(n)).all(|x| n % x != 0)
}

fn trunc_sqrt(n: u64) -> u64 {
  (n as f64).sqrt().trunc() as u64
}
