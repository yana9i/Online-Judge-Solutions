use core::num;
use std::{i32, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let chars: Vec<char> = input.trim().chars().collect();

    let mut operator_stack: Vec<char> = vec![];

    let mut number_stack: Vec<i32> = vec![];

    let mut last_operator_index: usize = 0;

    for i in 0..chars.len() {
        match chars[i] {
            '+' | '-' | '*' | '/' | '=' => {
                operator_stack.push(chars[i]);
                let last_number = input[last_operator_index..i].parse().unwrap();
                number_stack.push(last_number);
                last_operator_index = i + 1;
                if chars[i] == '=' {
                    break;
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                continue;
            }
            _ => {
                println!("ERROR");
                return;
            }
        }
    }

    for i in operator_stack {
        let first_num = *number_stack.first().unwrap();
        number_stack.remove(0);
        match i {
            '+' => {
                number_stack[0] += first_num;
            }
            '-' => {
                number_stack[0] = first_num - number_stack[0];
            }
            '*' => {
                number_stack[0] *= first_num;
            }
            '/' => {
                if number_stack[0] == 0 {
                    println!("ERROR");
                    return;
                }
                number_stack[0] = first_num / number_stack[0];
            }
            '=' => {
                println!("{}", first_num);
                return;
            }
            _ => {}
        }
    }
}
