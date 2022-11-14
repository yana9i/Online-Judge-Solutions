use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.trim().split(" ");
    let l = s.next().unwrap().parse::<usize>().unwrap();
    let k = s.next().unwrap().parse::<usize>().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let line = input.trim();
    let mut is_404 = true;
    if l >= k {
        for i in 0..=l - k {
            let num = u128::from_str_radix(&line[i..i + k], 10).unwrap();
            if is_prime(num) {
                let mut str = num.to_string();
                while str.len() < k {
                    str.insert(0, '0');
                }
                println!("{}", str);
                is_404 = false;
                break;
            }
        }
    }
    if is_404 {
        println!("404");
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n & 1 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u128;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
