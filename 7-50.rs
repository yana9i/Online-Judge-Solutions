use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();
    input.clear();
    let mut hash_map: HashMap<u32, u32> = HashMap::new();

    for n in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line = input.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        for &item in &line[1..] {
            let num = item.parse::<u32>().unwrap();

            if hash_map.get(&num).is_some() {
                hash_map.insert(num, hash_map.get(&num).unwrap() + 1);
            } else {
                hash_map.insert(num, 1);
            }
        }
    }

    let mut max_key = 0_u32;
    let mut max_val = 0_u32;
    for (k, v) in hash_map {
        if v > max_val {
            max_val = v;
            max_key = k;
        }
        if v == max_val && k > max_key {
            max_key = k;
        }
    }
    println!("{} {}", max_key, max_val);
}
