use crate::problems::Answer;
use crate::util::fibonacci;

#[allow(dead_code)]
pub fn problem() -> Answer {
    let fib = fibonacci(None, Some(4_000_000)).iter().cloned().filter(|&f| f % 2 == 0).collect::<Vec<u64>>();
    let ans: u64 = fib.iter().sum();

    Answer {
        answer: ans,
        context: format!("{:?}", fib)
    }
}
