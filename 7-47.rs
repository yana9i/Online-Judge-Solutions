use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut max_str = String::new();
    let mut max_len = 0;
    for n in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim();
        if n.len() > max_len {
            max_len = n.len();
            max_str = n.to_string();
        }
    }
    println!("The longest is: {}", max_str);
}
