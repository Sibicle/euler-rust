use crate::problems::Answer;

#[allow(dead_code)]
pub fn problem() -> Answer {
    let mut sum: u64 = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }

    Answer {
        answer: sum,
        context: String::from(""),
    }
}
