use core::num;
use std::{i32, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let int_iter = input
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap());

    let mut a_arr = [-1, -1, -1];
    let mut a3_sum = 0.0;

    for i in int_iter {
        match i % 3 {
            0 => {
                if i > a_arr[0] {
                    a_arr[0] = i;
                }
            }
            1 => {
                if a_arr[1] == -1 {
                    a_arr[1] = 1;
                } else {
                    a_arr[1] += 1;
                }
            }
            2 => {
                a3_sum += i as f64;
                if a_arr[2] == -1 {
                    a_arr[2] = 1;
                } else {
                    a_arr[2] += 1;
                }
            }
            _ => {}
        }
    }

    for (i, &item) in a_arr.iter().enumerate() {
        match item {
            -1 => {
                print!("NONE");
                if i != 2 {
                    print!(" ")
                }
            }
            _ => match i {
                0 => print!("{} ", item),
                1 => print!("{} ", item),
                2 => print!("{:.1}", a3_sum / item as f64),
                _ => {}
            },
        }
    }
}
