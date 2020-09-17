use crate::problems::Answer;


#[allow(dead_code)]
pub fn problem() -> Answer {
    const MAX_NUM: u64 = 1_000;
    let mut string = String::new();

    for i in 1..MAX_NUM + 1 {
        let s = num_to_text(i);
        string.push_str(&s);
        string.push_str("  ");
    }

    let ans_str = string.replace(" ", "").replace("-", "");

    Answer {
        answer: ans_str.len() as u64,
        context: num_to_text(MAX_NUM),
    }
}

fn num_to_text(num: u64) -> String {
    let mut string: String = String::new();
    let mut num_cur: u64 = num;
    let mut place: u64 = 0;

    while num_cur > 0 {
        if place > 33 {
            return String::from("a lot");
        }

        let hun = num_cur % 1000;

        if hun != 0 {
            let mut place_str: String = String::new();

            match place {
                1 => place_str.push_str(" thousand"),
                2 => place_str.push_str(" million"),
                3 => place_str.push_str(" billion"),
                4 => place_str.push_str(" trillion"),
                5 => place_str.push_str(" quadrillion"),
                6 => place_str.push_str(" quintillion"),
                7 => place_str.push_str(" sextillion"),
                8 => place_str.push_str(" septillion"),
                9 => place_str.push_str(" octillion"),
                10 => place_str.push_str(" nonillion"),
                11 => place_str.push_str(" decillion"),
                12 => place_str.push_str(" undecillion"),
                13 => place_str.push_str(" duodecillion"),
                14 => place_str.push_str(" tredecillion"),
                15 => place_str.push_str(" quattuordecillion"),
                16 => place_str.push_str(" quindecillion"),
                17 => place_str.push_str(" sexdecillion"),
                18 => place_str.push_str(" septemdecillion"),
                19 => place_str.push_str(" octodecillion"),
                20 => place_str.push_str(" novemdecillion"),
                21 => place_str.push_str(" vigintillion"),
                22 => place_str.push_str(" unvigintillion"),
                23 => place_str.push_str(" duovigintillion"),
                24 => place_str.push_str(" trevigintillion"),
                25 => place_str.push_str(" quattuorvigintillion"),
                26 => place_str.push_str(" quinvigintillion"),
                27 => place_str.push_str(" sexvigintillion"),
                28 => place_str.push_str(" septvigintillion"),
                29 => place_str.push_str(" octovigintillion"),
                30 => place_str.push_str(" nonvigintillion"),
                31 => place_str.push_str(" trigintillion"),
                32 => place_str.push_str(" untrigintillion"),
                33 => place_str.push_str(" duotrigintillion"),
                _ => (),
            }

            place_str.insert_str(0, &hun_to_text(hun));

            string.insert_str(0, &place_str);
            string.insert_str(0, " ");
        }

        num_cur /= 1000;
        place += 1;
    }

    string
}

fn hun_to_text(num: u64) -> String {
    let mut string = String::new();

    if num > 99 {
        string.push_str(&digit_to_text(num / 100));
        string.push_str(" hundred");

        if num % 100 != 0 {
            string.push_str(" and ");
        }
    }

    string.push_str(&tens_to_text(num % 100));

    string
}

fn tens_to_text(dig: u64) -> String {
    let mut string = String::new();

    if dig < 10 {
        string.push_str(&digit_to_text(dig));
    } else if dig == 10 {
        string.push_str("ten");
    } else if dig == 11 {
        string.push_str("eleven");
    } else if dig == 12 {
        string.push_str("twelve");
    } else if dig == 13 {
        string.push_str("thirteen");
    } else if dig == 14 {
        string.push_str("fourteen");
    } else if dig == 15 {
        string.push_str("fifteen");
    } else if dig == 16 {
        string.push_str("sixteen");
    } else if dig == 17 {
        string.push_str("seventeen");
    } else if dig == 18 {
        string.push_str("eighteen");
    } else if dig == 19 {
        string.push_str("nineteen");
    } else if dig > 19 {
        match dig / 10 {
            2 => string.push_str("twenty"),
            3 => string.push_str("thirty"),
            4 => string.push_str("forty"),
            5 => string.push_str("fifty"),
            6 => string.push_str("sixty"),
            7 => string.push_str("seventy"),
            8 => string.push_str("eighty"),
            9 => string.push_str("ninety"),
            _ => (),
        }

        if dig % 10 != 0 {
            string.push_str("-");
            string.push_str(&digit_to_text(dig % 10))
        }
    }

    return string;
}

fn digit_to_text(dig: u64) -> String {
    let mut string = String::new();

    match dig {
        1 => string.push_str("one"),
        2 => string.push_str("two"),
        3 => string.push_str("three"),
        4 => string.push_str("four"),
        5 => string.push_str("five"),
        6 => string.push_str("six"),
        7 => string.push_str("seven"),
        8 => string.push_str("eight"),
        9 => string.push_str("nine"),
        _ => (),
    }

    string
}
