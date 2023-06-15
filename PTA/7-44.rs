use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut str = "".to_string();
    let n = input.trim().chars();
    let mut hex_start_flag = false;
    let mut is_neg = 1;
    for i in n {
        if !hex_start_flag && i == '-' {
            is_neg = -1;
        }
        if i.is_digit(16) {
            str.push(i);
            hex_start_flag = true;
        }
    }

    let res = match i128::from_str_radix(&str, 16) {
        Ok(i) => i * is_neg,
        Err(_) => 0,
    };

    println!("{}", res);
}
