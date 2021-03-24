pub fn is_armstrong_number(num: u32) -> bool {
  let len = (num as f64).log10() as u32 + 1;
  num == (0..len).map(|e| (num / 10u32.pow(e) % 10).pow(len)).sum()
}
