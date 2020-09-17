use crate::problems::Answer;

#[allow(dead_code)]
fn problem() -> Answer {
    let num: u32 = paths(10, 10, 10);

    Answer {
        answer: num as u64,
        context: String::from("none"),
    }
}

fn paths(width: u32, height: u32, _this_path: u32) -> u32 {
    let mut num_paths: u32 = 0;

    if width > 0 {
        num_paths += paths(width - 1, height, num_paths);
    }

    if height > 0 {
        num_paths += paths(width, height - 1, num_paths);
    }

    return num_paths;
}
