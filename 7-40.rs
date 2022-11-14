use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.trim().split(" ");
    let mut n = s.next().unwrap().parse::<u32>().unwrap();
    let ch = s.next().unwrap();
    let mut line_count_list: Vec<u32> = vec![];
    let mut line_symbol_count = 1_u32;
    if n == 0 {
        println!();
    } else if n < 7 {
        line_count_list.push(1);
        n -= 1;
    } else {
        line_count_list.push(1);
        n -= 1;
        while line_symbol_count * 2 < n {
            line_symbol_count += 2;
            line_count_list.insert(0, line_symbol_count);
            line_count_list.push(line_symbol_count);
            n -= (line_symbol_count * 2);
        }
    }
    for i in line_count_list.clone() {
        let mut space_counter = (line_count_list[0] - i) / 2;
        let mut symbol_counter = i;
        while space_counter > 0 {
            space_counter -= 1;
            print!(" ");
        }
        while symbol_counter > 0 {
            symbol_counter -= 1;
            print!("{}", ch);
        }
        println!();
    }
    println!("{}", n);
}
