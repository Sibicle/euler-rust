use crate::problems::Answer;

use num::bigint::BigUint;

#[allow(dead_code)]
fn problem() -> Answer {
  let number_str = BigUint::from(2u32).pow(1000).to_string();
  let sum: u32 = number_str.chars().map(|c| c.to_digit(10).unwrap()).sum();

  Answer {
      answer: sum as u64,
      context: number_str,
  }
}
