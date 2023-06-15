use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let mut int_vec: Vec<i32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    int_vec.sort();

    int_vec.reverse();

    println!(
        "{}",
        int_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
