use crate::problems::Answer;

#[allow(dead_code)]
pub fn problem() -> Answer {
    let mut max: u64 = 0;

    for x in 1..999 {
        for y in (1..999).rev() {
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

            if palie && pal > max { max = pal }
        }
    }

    Answer {
        answer: max,
        context: String::from("")
    }
}
