use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    let mut addr_book: Vec<Vec<String>> = vec![];

    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line_vec: Vec<String> = line.trim().split(" ").map(|x| x.to_string()).collect();
        addr_book.push(line_vec);
    }

    addr_book.sort_by(|a, b| a[1].cmp(&b[1]));

    for i in addr_book {
        println!("{}", i.join(" "));
    }
}
