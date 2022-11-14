use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let mut counter_arr: [u32; 10] = [0; 10];

    for i in input.bytes() {
        if b'0' <= i && i <= b'9' {
            counter_arr[(i - b'0') as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut res_str = String::new();

    for (i, &item) in counter_arr.iter().enumerate() {
        if max_count < item {
            res_str.clear();
            max_count = item;
            res_str = format!(" {}", i);
        } else if max_count == item {
            res_str = format!("{} {}", res_str, i);
        }
    }

    println!("{}:{}", max_count, res_str);
}