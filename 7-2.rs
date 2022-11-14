use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    match n {
        2001..=2003 => {
            println!("None")
        }
        2004..=2100 => {
            let mut i = 2004;
            loop {
                println!("{}", i);
                i += 4;
                if i > n || i == 2100 {
                    break;
                }
            }
        }
        _ => {
            println!("Invalid year!");
        }
    }
}