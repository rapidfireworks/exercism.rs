pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let fs: Vec<_> = factors.iter().filter(|&&f| 0 < f).copied().collect();
  (1..limit).filter(|&m| is_multiple(m, &fs)).sum()
}

fn is_multiple(dividend: u32, divisors: &[u32]) -> bool {
  divisors.iter().any(|divisor| dividend % divisor == 0)
}
