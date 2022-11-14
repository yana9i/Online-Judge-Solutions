use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    let mut sum: f64 = 0.0;

    for i in 1..n + 1 {
        let s: i32 = (i + 1) & 1;
        sum += (-1 as i32).pow(s as u32) as f64 * 1.0 / (3 * i - 2) as f64;
    }
    println!("sum = {:.3}", sum);
}