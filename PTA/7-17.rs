use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut int_vec: Vec<String> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<String>().unwrap())
        .collect();

    int_vec.sort();

    println!("After sorted:");

    println!(
        "{}",
        int_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
