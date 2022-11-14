use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();

    let week = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    input.clear();

    for n in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim();
        match week.iter().position(|&x| x == n) {
            Some(i) => println!("{}", i + 1),
            None => println!("-1"),
        }
    }
}
