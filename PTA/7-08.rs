use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let chars: Vec<char> = input.trim().chars().collect();

    let n = chars.len();

    for i in 0..n {
        print!("{} ", chars[i]);
    }
}
