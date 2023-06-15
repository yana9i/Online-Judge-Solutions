use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let line_1: Vec<i32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let mut line_2: Vec<i32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    line_2.reverse();

    println!(
        "{}",
        line_2
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
