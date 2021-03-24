pub fn is_armstrong_number(num: u32) -> bool {
  let len = (num as f64).log10() as u32 + 1;
  let mut sum = 0;
  let mut dividend = num;
  for divisor in (0..len).map(|e| 10u32.pow(e)).rev() {
    sum += (dividend / divisor).pow(len);
    dividend %= divisor;
  }
  sum == num
}
