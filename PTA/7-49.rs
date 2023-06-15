use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().as_bytes();
    let mut digits = n.len() as f64;
    let mut is_neg = 1.0;
    let mut is_odd = 1.0;
    let mut two_counter = 0;
    if n[0] == b'-' {
        digits -= 1.0;
        is_neg = 1.5;
    }
    if n.last().unwrap() - b'0' & 1 == 0 {
        is_odd = 2.0;
    }
    for i in n {
        if i == &b'2' {
            two_counter += 1;
        }
    }
    println!(
        "{:.2}%",
        two_counter as f64 / digits * is_neg * is_odd * 100.0
    );
}
