use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let line: Vec<u32> = input
        .trim()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut k = line[1];

    let mut sum = 0_u32;

    let mut res_vec: Vec<String> = vec![];

    for i in (0..line[0] + 1).rev() {
        if is_prime(i) {
            sum += i;
            k -= 1;
            res_vec.push(i.to_string());
            if k == 0 {
                break;
            }
        }
    }

    println!("{}={}", res_vec.join("+"), sum);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
