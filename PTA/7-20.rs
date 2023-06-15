use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<i32>().unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let mut min = f64::MAX;
    let mut max = f64::MIN;
    let mut sum = 0.0;

    for i in numbers {
        if i < min {
            min = i;
        }
        if i > max {
            max = i;
        }
        sum += i;
    }

    println!("average = {:.2}", sum / n as f64);
    println!("max = {:.2}", max);
    println!("min = {:.2}", min);
}
