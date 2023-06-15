use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();
    let mut has_mer_num = false;
    for i in 0..n + 1 {
        if i < 2 {
            continue;
        }
        let mer_num = 2_u32.pow(i) - 1;
        if is_prime(mer_num) {
            println!("{}", mer_num);
            has_mer_num = true;
        }
    }
    if !has_mer_num {
        println!("None");
    }
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