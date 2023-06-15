use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut num: u32 = input.trim().parse().unwrap();
        println!("{}", factoring(2, num));
    }
}

fn factoring(a: u32, b: u32) -> u32 {
    if b == 1 {
        return 1;
    }
    let mut res = 0_u32;
    for i in a..=b {
        if b % i == 0 {
            res += factoring(i, b / i);
        }
    }

    res
}
