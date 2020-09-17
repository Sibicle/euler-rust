use elapsed::measure_time;

mod problems;
mod util;

fn main() {
    let (elapsed, answer) = measure_time(|| problems::seventeen::problem());

    println!("elapsed: {}", elapsed);
    println!("answer: {}", answer.answer);
    println!("context: {}", answer.context);
}
