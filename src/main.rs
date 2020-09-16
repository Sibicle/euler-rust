extern crate num;

use elapsed::measure_time;
use num::bigint::{BigUint, ToBigUint};

fn main() {
    let (elapsed, answer) = measure_time(|| seventeen(1_000));

    println!("elapsed: {}", elapsed);
    println!("answer: {}", answer.answer);
    println!("context: {}", answer.context);
}

fn seventeen(num: usize) -> Answer {
    let mut string = String::new();

    for i in 1..num + 1 {
        let s = num_to_text(i);
        println!("{} {}",i, s);
        string.push_str(&s);
        string.push_str("  ");
    }

    let ans_str = string.replace(" ", "").replace("-", "");
    println!("'{}'", ans_str);

    Answer {
        answer: ans_str.len(),
        context: num_to_text(1_234_618),
    }
}

fn num_to_text(num: usize) -> String {
    let mut string: String = String::new();
    let mut num_cur: usize = num;
    let mut place: usize = 0;

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

fn hun_to_text(num: usize) -> String {
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

fn tens_to_text(dig: usize) -> String {
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

fn digit_to_text(dig: usize) -> String {
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

#[allow(dead_code)]
fn sixteen() -> Answer {
    let number_str = BigUint::from(2u32).pow(1000).to_string();
    let sum: u32 = number_str.chars().map(|c| c.to_digit(10).unwrap()).sum();

    Answer {
        answer: sum as usize,
        context: number_str,
    }
}

#[allow(dead_code)]
fn fifteen() -> Answer {
    let num: usize = paths(10, 10, 10);

    Answer {
        answer: num,
        context: String::from("none"),
    }
}

fn paths(width: usize, height: usize, _this_path: usize) -> usize {
    let mut num_paths: usize = 0;

    if width > 0 {
        num_paths += paths(width - 1, height, num_paths);
    }

    if height > 0 {
        num_paths += paths(width, height - 1, num_paths);
    }

    return num_paths;
}

struct Answer {
    answer: usize,
    context: String,
}

#[allow(dead_code)]
fn fourteen() -> Answer {
    const START: usize = 1_000_000;
    let mut terms: usize = 0;
    let mut num: usize = 0;

    for i in 1..START {
        let t = collatz_terms(i);
        if t > terms {
            terms = terms;
            num = i;
        }
    }

    Answer {
        answer: num,
        context: terms.to_string(),
    }
}

fn collatz_terms(num: usize) -> usize {
    let mut terms: usize = 1;
    let mut new_num: usize = num;

    loop {
        if new_num <= 1 {
            break;
        }

        if new_num % 2 == 0 {
            new_num = new_num / 2;
        } else {
            new_num = (new_num * 3) + 1;
        }

        terms = terms + 1;
    }

    terms
}

fn _thirteen() {
    let nums: Vec<BigUint> = vec![
        BigUint::parse_bytes(b"37107287533902102798797998220837590246510135740250", 10).unwrap(),
        BigUint::parse_bytes(b"46376937677490009712648124896970078050417018260538", 10).unwrap(),
        BigUint::parse_bytes(b"74324986199524741059474233309513058123726617309629", 10).unwrap(),
        BigUint::parse_bytes(b"91942213363574161572522430563301811072406154908250", 10).unwrap(),
        BigUint::parse_bytes(b"23067588207539346171171980310421047513778063246676", 10).unwrap(),
        BigUint::parse_bytes(b"89261670696623633820136378418383684178734361726757", 10).unwrap(),
        BigUint::parse_bytes(b"28112879812849979408065481931592621691275889832738", 10).unwrap(),
        BigUint::parse_bytes(b"44274228917432520321923589422876796487670272189318", 10).unwrap(),
        BigUint::parse_bytes(b"47451445736001306439091167216856844588711603153276", 10).unwrap(),
        BigUint::parse_bytes(b"70386486105843025439939619828917593665686757934951", 10).unwrap(),
        BigUint::parse_bytes(b"62176457141856560629502157223196586755079324193331", 10).unwrap(),
        BigUint::parse_bytes(b"64906352462741904929101432445813822663347944758178", 10).unwrap(),
        BigUint::parse_bytes(b"92575867718337217661963751590579239728245598838407", 10).unwrap(),
        BigUint::parse_bytes(b"58203565325359399008402633568948830189458628227828", 10).unwrap(),
        BigUint::parse_bytes(b"80181199384826282014278194139940567587151170094390", 10).unwrap(),
        BigUint::parse_bytes(b"35398664372827112653829987240784473053190104293586", 10).unwrap(),
        BigUint::parse_bytes(b"86515506006295864861532075273371959191420517255829", 10).unwrap(),
        BigUint::parse_bytes(b"71693888707715466499115593487603532921714970056938", 10).unwrap(),
        BigUint::parse_bytes(b"54370070576826684624621495650076471787294438377604", 10).unwrap(),
        BigUint::parse_bytes(b"53282654108756828443191190634694037855217779295145", 10).unwrap(),
        BigUint::parse_bytes(b"36123272525000296071075082563815656710885258350721", 10).unwrap(),
        BigUint::parse_bytes(b"45876576172410976447339110607218265236877223636045", 10).unwrap(),
        BigUint::parse_bytes(b"17423706905851860660448207621209813287860733969412", 10).unwrap(),
        BigUint::parse_bytes(b"81142660418086830619328460811191061556940512689692", 10).unwrap(),
        BigUint::parse_bytes(b"51934325451728388641918047049293215058642563049483", 10).unwrap(),
        BigUint::parse_bytes(b"62467221648435076201727918039944693004732956340691", 10).unwrap(),
        BigUint::parse_bytes(b"15732444386908125794514089057706229429197107928209", 10).unwrap(),
        BigUint::parse_bytes(b"55037687525678773091862540744969844508330393682126", 10).unwrap(),
        BigUint::parse_bytes(b"18336384825330154686196124348767681297534375946515", 10).unwrap(),
        BigUint::parse_bytes(b"80386287592878490201521685554828717201219257766954", 10).unwrap(),
        BigUint::parse_bytes(b"78182833757993103614740356856449095527097864797581", 10).unwrap(),
        BigUint::parse_bytes(b"16726320100436897842553539920931837441497806860984", 10).unwrap(),
        BigUint::parse_bytes(b"48403098129077791799088218795327364475675590848030", 10).unwrap(),
        BigUint::parse_bytes(b"87086987551392711854517078544161852424320693150332", 10).unwrap(),
        BigUint::parse_bytes(b"59959406895756536782107074926966537676326235447210", 10).unwrap(),
        BigUint::parse_bytes(b"69793950679652694742597709739166693763042633987085", 10).unwrap(),
        BigUint::parse_bytes(b"41052684708299085211399427365734116182760315001271", 10).unwrap(),
        BigUint::parse_bytes(b"65378607361501080857009149939512557028198746004375", 10).unwrap(),
        BigUint::parse_bytes(b"35829035317434717326932123578154982629742552737307", 10).unwrap(),
        BigUint::parse_bytes(b"94953759765105305946966067683156574377167401875275", 10).unwrap(),
        BigUint::parse_bytes(b"88902802571733229619176668713819931811048770190271", 10).unwrap(),
        BigUint::parse_bytes(b"25267680276078003013678680992525463401061632866526", 10).unwrap(),
        BigUint::parse_bytes(b"36270218540497705585629946580636237993140746255962", 10).unwrap(),
        BigUint::parse_bytes(b"24074486908231174977792365466257246923322810917141", 10).unwrap(),
        BigUint::parse_bytes(b"91430288197103288597806669760892938638285025333403", 10).unwrap(),
        BigUint::parse_bytes(b"34413065578016127815921815005561868836468420090470", 10).unwrap(),
        BigUint::parse_bytes(b"23053081172816430487623791969842487255036638784583", 10).unwrap(),
        BigUint::parse_bytes(b"11487696932154902810424020138335124462181441773470", 10).unwrap(),
        BigUint::parse_bytes(b"63783299490636259666498587618221225225512486764533", 10).unwrap(),
        BigUint::parse_bytes(b"67720186971698544312419572409913959008952310058822", 10).unwrap(),
        BigUint::parse_bytes(b"95548255300263520781532296796249481641953868218774", 10).unwrap(),
        BigUint::parse_bytes(b"76085327132285723110424803456124867697064507995236", 10).unwrap(),
        BigUint::parse_bytes(b"37774242535411291684276865538926205024910326572967", 10).unwrap(),
        BigUint::parse_bytes(b"23701913275725675285653248258265463092207058596522", 10).unwrap(),
        BigUint::parse_bytes(b"29798860272258331913126375147341994889534765745501", 10).unwrap(),
        BigUint::parse_bytes(b"18495701454879288984856827726077713721403798879715", 10).unwrap(),
        BigUint::parse_bytes(b"38298203783031473527721580348144513491373226651381", 10).unwrap(),
        BigUint::parse_bytes(b"34829543829199918180278916522431027392251122869539", 10).unwrap(),
        BigUint::parse_bytes(b"40957953066405232632538044100059654939159879593635", 10).unwrap(),
        BigUint::parse_bytes(b"29746152185502371307642255121183693803580388584903", 10).unwrap(),
        BigUint::parse_bytes(b"41698116222072977186158236678424689157993532961922", 10).unwrap(),
        BigUint::parse_bytes(b"62467957194401269043877107275048102390895523597457", 10).unwrap(),
        BigUint::parse_bytes(b"23189706772547915061505504953922979530901129967519", 10).unwrap(),
        BigUint::parse_bytes(b"86188088225875314529584099251203829009407770775672", 10).unwrap(),
        BigUint::parse_bytes(b"11306739708304724483816533873502340845647058077308", 10).unwrap(),
        BigUint::parse_bytes(b"82959174767140363198008187129011875491310547126581", 10).unwrap(),
        BigUint::parse_bytes(b"97623331044818386269515456334926366572897563400500", 10).unwrap(),
        BigUint::parse_bytes(b"42846280183517070527831839425882145521227251250327", 10).unwrap(),
        BigUint::parse_bytes(b"55121603546981200581762165212827652751691296897789", 10).unwrap(),
        BigUint::parse_bytes(b"32238195734329339946437501907836945765883352399886", 10).unwrap(),
        BigUint::parse_bytes(b"75506164965184775180738168837861091527357929701337", 10).unwrap(),
        BigUint::parse_bytes(b"62177842752192623401942399639168044983993173312731", 10).unwrap(),
        BigUint::parse_bytes(b"32924185707147349566916674687634660915035914677504", 10).unwrap(),
        BigUint::parse_bytes(b"99518671430235219628894890102423325116913619626622", 10).unwrap(),
        BigUint::parse_bytes(b"73267460800591547471830798392868535206946944540724", 10).unwrap(),
        BigUint::parse_bytes(b"76841822524674417161514036427982273348055556214818", 10).unwrap(),
        BigUint::parse_bytes(b"97142617910342598647204516893989422179826088076852", 10).unwrap(),
        BigUint::parse_bytes(b"87783646182799346313767754307809363333018982642090", 10).unwrap(),
        BigUint::parse_bytes(b"10848802521674670883215120185883543223812876952786", 10).unwrap(),
        BigUint::parse_bytes(b"71329612474782464538636993009049310363619763878039", 10).unwrap(),
        BigUint::parse_bytes(b"62184073572399794223406235393808339651327408011116", 10).unwrap(),
        BigUint::parse_bytes(b"66627891981488087797941876876144230030984490851411", 10).unwrap(),
        BigUint::parse_bytes(b"60661826293682836764744779239180335110989069790714", 10).unwrap(),
        BigUint::parse_bytes(b"85786944089552990653640447425576083659976645795096", 10).unwrap(),
        BigUint::parse_bytes(b"66024396409905389607120198219976047599490197230297", 10).unwrap(),
        BigUint::parse_bytes(b"64913982680032973156037120041377903785566085089252", 10).unwrap(),
        BigUint::parse_bytes(b"16730939319872750275468906903707539413042652315011", 10).unwrap(),
        BigUint::parse_bytes(b"94809377245048795150954100921645863754710598436791", 10).unwrap(),
        BigUint::parse_bytes(b"78639167021187492431995700641917969777599028300699", 10).unwrap(),
        BigUint::parse_bytes(b"15368713711936614952811305876380278410754449733078", 10).unwrap(),
        BigUint::parse_bytes(b"40789923115535562561142322423255033685442488917353", 10).unwrap(),
        BigUint::parse_bytes(b"44889911501440648020369068063960672322193204149535", 10).unwrap(),
        BigUint::parse_bytes(b"41503128880339536053299340368006977710650566631954", 10).unwrap(),
        BigUint::parse_bytes(b"81234880673210146739058568557934581403627822703280", 10).unwrap(),
        BigUint::parse_bytes(b"82616570773948327592232845941706525094512325230608", 10).unwrap(),
        BigUint::parse_bytes(b"22918802058777319719839450180888072429661980811197", 10).unwrap(),
        BigUint::parse_bytes(b"77158542502016545090413245809786882778948721859617", 10).unwrap(),
        BigUint::parse_bytes(b"72107838435069186155435662884062257473692284509516", 10).unwrap(),
        BigUint::parse_bytes(b"20849603980134001723930671666823555245252804609722", 10).unwrap(),
        BigUint::parse_bytes(b"53503534226472524250874054075591789781264330331690", 10).unwrap(),
    ];

    let mut sum: BigUint = (0 as u8).to_biguint().unwrap();

    for num in nums.iter() {
        sum = sum + num;
    }

    let sum_str = sum.to_str_radix(10);

    println!("{}", &sum_str[..10]);
}

fn _twelve() {
    let mut tri = 1;
    let mut i = 2;
    let mut fac = 2;

    loop {
        tri = tri + i;
        i = i + 1;

        let new_fac = _factor(tri).len();

        if new_fac > fac {
            fac = new_fac
        }

        if fac > 500 {
            break;
        }
        println!("{} {}", tri, fac);
    }

    println!("{}", tri);
}

fn _factor(num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new(); // creates a new vector for the factors of the number

    for i in 1..((num as f32).sqrt() as i32 + 1) {
        if num % i == 0 {
            factors.push(i); // pushes smallest factor to factors
            factors.push(num / i); // pushes largest factor to factors
        }
    }
    factors.sort(); // sorts the factors into numerical order for viewing purposes
    factors // returns the factors
}

fn _sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false
    }
    let sqrtlmt = (limit as f64).sqrt() as usize + 1;

    for num in 2..sqrtlmt {
        if is_prime[num] {
            let mut multiple = num * num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr { Some(pr) } else { None })
        .collect()
}

fn _eleven() {
    let arr = [
        [
            08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
        ],
        [
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
        ],
        [
            81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
        ],
        [
            52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
        ],
        [
            22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
        ],
        [
            24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
        ],
        [
            32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
        ],
        [
            67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
        ],
        [
            24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
        ],
        [
            21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
        ],
        [
            78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
        ],
        [
            16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
        ],
        [
            86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
        ],
        [
            19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
        ],
        [
            04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
        ],
        [
            88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
        ],
        [
            04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
        ],
        [
            20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
        ],
        [
            20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
        ],
        [
            01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
        ],
    ];

    let mut prod: usize = 0;

    // vertical
    for i in 0..17 {
        for j in 0..20 {
            let this = arr[i][j] * arr[i + 1][j] * arr[i + 2][j] * arr[i + 3][j];

            if this > prod {
                prod = this
            }
        }
    }

    // horizontal
    for i in 0..20 {
        for j in 0..17 {
            let this = arr[i][j] * arr[i][j + 1] * arr[i][j + 2] * arr[i][j + 3];

            if this > prod {
                prod = this
            }
        }
    }

    // diagonal
    for i in 0..17 {
        for j in 0..17 {
            let this = arr[i][j] * arr[i + 1][j + 1] * arr[i + 2][j + 2] * arr[i + 3][j + 3];

            if this > prod {
                prod = this
            }
        }
    }

    // other diagonal
    for i in 3..17 {
        for j in 0..17 {
            let this = arr[i][j] * arr[i - 1][j + 1] * arr[i - 2][j + 2] * arr[i - 3][j + 3];

            if this > prod {
                prod = this
            }
        }
    }

    println!("{}", prod);
}

fn _one() {
    let mut sum: usize = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum = sum + i;
        }
    }

    println!("{}", sum);
}
