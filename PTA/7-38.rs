use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    let mut iter = 0_u32;

    loop {
        let new_n = handle(n);
        iter += 1;
        if n == new_n {
            println!("{}:{}", iter, new_n);
            break;
        } else {
            n = new_n;
        }
        println!("{}:{}", iter, new_n);
    }
}

fn handle(num: u32) -> u32 {
    let mut num = num;
    let mut res = 0_u32;

    while num > 10 {
        res += num % 10;
        num /= 10;
    }

    res += num;

    res * 3 + 1
}
