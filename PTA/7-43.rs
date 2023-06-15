use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let m = line[0];
    let n = line[1];

    let mut matrix: Vec<Vec<i32>> = vec![];

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(line);
    }

    for line in matrix {
        let mut line_clone = line.clone();
        line_clone.rotate_right((m % n) as usize);
        println!(
            "{}",
            line_clone
                .iter()
                .map(|&x| format!("{} ", x))
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
