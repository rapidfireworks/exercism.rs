pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let factors: Vec<&u32> = factors.iter().filter(|&&x| 0 < x).collect();
  (1..limit).filter(|x| is_multiple(x, &factors)).sum()
}

fn is_multiple(dividend: &u32, divisors: &[&u32]) -> bool {
  divisors
    .iter()
    .filter(|&&x| dividend % x == 0)
    .next()
    .is_some()
}
