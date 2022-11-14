use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let mut split_str = input.trim().split(' ');

    let mut n1: i32 = split_str.next().unwrap().parse().unwrap();

    for i in 1..n {
        let n2 = split_str.next().unwrap().parse().unwrap();
        print!("{}", n2 - n1);
        n1 = n2;
        if i != n - 1 {
            if i % 3 == 0 {
                println!();
            } else {
                print!(" ")
            }
        }
    }
}