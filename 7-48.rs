use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let line: Vec<u32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = line[0];
    let k = line[1];

    let mut string_vec: Vec<String> = vec![];

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        string_vec.push(input.trim().to_string());
    }

    for i in 0..k {
        bubble_sort(&mut string_vec);
    }

    for i in string_vec {
        println!("{}", i);
    }
}

fn bubble_sort(vec: &mut Vec<String>) -> &Vec<String> {
    for i in 1..vec.len() {
        let mut a: &String = &vec[i - 1];
        let mut b: &String = &vec[i];

        let a_val = a.to_string();
        let b_val = b.to_string();

        if a_val.gt(&b_val) {
            vec.swap(i - 1, i);
        }
    }

    vec
}
