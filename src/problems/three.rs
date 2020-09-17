use crate::problems::Answer;
use crate::util::factors;
use crate::util::is_prime;

#[allow(dead_code)]
pub fn problem() -> Answer {
    let fac = factors(600_851_475_143);

    let mut max: u64 = 0;

    for num in fac.iter() {
        if is_prime(*num) && num > &max { max = *num}
    }

    Answer {
        answer: max,
        context: format!("{:?}", fac)
    }
}
