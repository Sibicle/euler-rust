use crate::problems::Answer;

#[allow(dead_code)]
fn problem() -> Answer {
  const START: u64 = 1_000_000;
  let mut terms: u64 = 0;
  let mut num: u64 = 0;

  for i in 1..START {
      let t = collatz_terms(i);
      if t > terms {
          terms = terms;
          num = i;
      }
  }

  Answer {
      answer: num,
      context: terms.to_string(),
  }
}

fn collatz_terms(num: u64) -> u64 {
  let mut terms: u64 = 1;
  let mut new_num: u64 = num;

  loop {
      if new_num <= 1 {
          break;
      }

      if new_num % 2 == 0 {
          new_num = new_num / 2;
      } else {
          new_num = (new_num * 3) + 1;
      }

      terms = terms + 1;
  }

  terms
}
