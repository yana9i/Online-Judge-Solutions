use std::io;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    let mut iter = 0_u32;

    loop {
        iter += 1;
        print!("{}: ", iter);
        n = handle(n);
        if n == 495 {
            break;
        }
    }
}

fn handle(num: u32) -> u32 {
    let mut char_vec: Vec<char> = num.to_string().chars().collect();
    char_vec.sort();
    let min_num: u32 = String::from_iter(char_vec.to_owned()).parse().unwrap();
    char_vec.reverse();
    let max_num: u32 = String::from_iter(char_vec).parse().unwrap();
    let res = max_num - min_num;
    println!("{} - {} = {}", max_num, min_num, res);

    res
}
