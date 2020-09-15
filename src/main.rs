fn main() {
    one();
}

fn one() {
    let mut sum: usize = 0;

    for i in 1..1000 {
        if i%3 == 0 || i%5 == 0 {
            sum = sum + i;
        }
    }

    println!("{}", sum);
}
