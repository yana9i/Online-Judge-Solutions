use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut sum: u128 = 0;
    let mut nt: u128 = 1;
    let n: usize = input.trim().parse().unwrap();
    for i in 1..=n {
        nt *= i as u128;
        sum += nt;
    }
    println!("{}", sum);
}
