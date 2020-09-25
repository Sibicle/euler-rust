use crate::problems::Answer;

#[allow(dead_code)]
pub fn problem() -> Answer {
    let mut max: u64 = 0;
    let mut x_max: u64 = 0;
    let mut y_max: u64 = 0;

    for x in (1..999).rev() {
        for y in (1..999).rev() {
            if max != 0 && x * y < max { break; }

            let pal = x * y;
            let pal_str = (x * y).to_string();
            let len = pal_str.len();
            let mut palie = true;

            for i in 0..(len/2) {
                let start = pal_str.chars().nth(i).unwrap();
                let end = pal_str.chars().nth(len - i - 1).unwrap();

                if start != end {
                    palie = false;
                    break;
                }
            }

            if palie && pal > max {
                max = pal;
                x_max = x;
                y_max = y;
            }
        }
    }

    Answer {
        answer: max,
        context: format!("{} x {}", x_max, y_max)
    }
}
