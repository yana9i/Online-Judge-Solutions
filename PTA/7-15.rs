use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut s = input.trim().split(" ");

    let m = s.next().unwrap().parse::<u32>().unwrap();
    let n = s.next().unwrap().parse::<u32>().unwrap();

    let mut matrix: Vec<Vec<i32>> = vec![];

    for i in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<i32> = input
            .trim()
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        matrix.push(line);
    }

    let mut has_output = false;

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let i = i as usize;
            let j = j as usize;
            let num = matrix[i][j];
            if num > matrix[i - 1][j]
                && num > matrix[i + 1][j]
                && num > matrix[i][j + 1]
                && num > matrix[i][j - 1]
            {
                println!("{} {} {}", num, i + 1, j + 1);
                has_output = true;
            }
        }
    }

    if !has_output {
        println!("None {} {}", m, n);
    }
}
