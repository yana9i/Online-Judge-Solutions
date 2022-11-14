use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut counter = [0_u32; 10];
    let n = input.trim().chars();
    for i in n {
        counter[(i as usize - '0' as usize)] += 1;
    }
    for (i, item) in counter.iter().enumerate() {
        if *item != 0 {
            println!("{}:{}", i, item);
        }
    }
}
