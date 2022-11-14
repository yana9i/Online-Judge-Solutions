use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let threshold = input.trim().parse::<f64>().unwrap();

    let mut sum = 0.0;

    let mut iter = 1.0;

    let mut n: u128 = 1;

    let mut a: u128 = 1;

    let mut b: u128 = 1;

    while iter > threshold {
        iter = a as f64 / b as f64;
        sum += iter;
        a = a * n;
        b = b * (2 * n + 1);
        n += 1;
    }

    println!("{:.6}", sum * 2.0);
}