use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let line: Vec<u32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut prime_vec: Vec<u32> = (line[0]..=line[1]).collect();

    prime_vec = prime_vec
        .iter()
        .filter(|&&x| is_prime(x))
        .map(|x| *x)
        .collect();

    let lenth = prime_vec.len();
    let mut counter = 0;

    for p in 0..lenth {
        for q in p + 1..lenth {
            for r in q + 1..lenth {
                let a = prime_vec[p] * prime_vec[q] + prime_vec[r];
                let b = prime_vec[q] * prime_vec[r] + prime_vec[p];
                let c = prime_vec[r] * prime_vec[p] + prime_vec[q];
                if is_prime(a) && is_prime(b) && is_prime(c) {
                    counter += 1;
                }
            }
        }
    }

    println!("{}", counter);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
