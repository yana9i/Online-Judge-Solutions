use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    let mut max = f64::MIN;

    let mut min = f64::MAX;

    let mut max_book = String::new();

    let mut min_book = String::new();

    for i in 0..n {
        let mut line_book = String::new();
        io::stdin().read_line(&mut line_book).unwrap();
        let mut line_price = String::new();
        io::stdin().read_line(&mut line_price).unwrap();
        let mut line_price: f64 = line_price.trim().parse::<f64>().unwrap();
        if line_price < min {
            min = line_price;
            min_book = line_book.clone();
        }
        if (line_price > max) {
            max = line_price;
            max_book = line_book.clone();
        }
    }

    println!("{:.2}, {}", max, max_book.trim());
    println!("{:.2}, {}", min, min_book.trim());
}
