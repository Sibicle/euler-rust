pub fn factors(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    for i in 1..((num as f32).sqrt() as u64 + 1) {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors.sort();
    factors
}

#[allow(dead_code)]
pub fn sieve(limit: usize) -> Vec<usize> {
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

pub fn is_prime(num: u64) -> bool {
    let sqrtlmt = (num as f64).sqrt() as usize + 1;
    for i in 2..sqrtlmt {
        if num % i as u64 == 0 { return false }
    }
    true
}

pub fn fibonacci(terms: Option<u64>, max: Option<u64>) -> Vec<u64> {
    let mut fib: Vec<u64> = vec![1];

    let mut n_0 = 1;
    let mut n = 1;

    loop {
        if terms != None && fib.len() >= terms.unwrap() as usize {
            break;
        }
        if max != None && n > max.unwrap() {
            break;
        }

        fib.push(n);

        let next = n_0 + n;
        n_0 = n;
        n = next;
    }

    fib
}
