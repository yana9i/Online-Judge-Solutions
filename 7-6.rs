use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let li = input.split_whitespace();

    println!("{}", li.count());
}
