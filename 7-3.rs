use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = String::from(input.trim());

    let mut res = 0;

    for i in n.bytes() {
        res += i - '0' as u8;
    }

    println!("{} {}", n.len(), res);
}
