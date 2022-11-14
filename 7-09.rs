use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let mut remaining_cash: i32 = n - 8;

    let mut counter = 0;

    for i in (0..remaining_cash / 5 + 1).rev() {
        let remaining_cash_by5 = remaining_cash - 5 * i;
        for j in (0..remaining_cash_by5 / 2 + 1).rev() {
            let remaining_cash_by2 = remaining_cash_by5 - 2 * j;
            println!(
                "fen5:{}, fen2:{}, fen1:{}, total:{}",
                i + 1,
                j + 1,
                remaining_cash_by2 + 1,
                i + j + remaining_cash_by2 + 3
            );
            counter += 1;
        }
    }

    println!("count = {}", counter);
}
