use crate::problems::Answer;
use crate::util::factors;

#[allow(dead_code)]
fn problem() -> Answer {
  let mut tri = 1;
  let mut i = 2;
  let mut fac = 2;

  loop {
      tri = tri + i;
      i = i + 1;

      let new_fac = factors(tri).len();

      if new_fac > fac {
          fac = new_fac
      }

      if fac > 500 {
          break;
      }
      println!("{} {}", tri, fac);
  }

  Answer {
      answer: tri,
      context: String::from("")
  }
}
