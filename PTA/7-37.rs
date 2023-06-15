use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.trim().split(" ");
    let m = s.next().unwrap().parse::<usize>().unwrap();
    let n = s.next().unwrap().parse::<usize>().unwrap();
    let mut flag = true;
    for i in m..=n {
        if handle(i) {
            flag = false;
        }
    }
    if flag {
        println!("None");
    }
}

fn handle(num: usize) -> bool {
    let mut factor: Vec<String> = vec!["1".to_string()];
    let mut sum = 1_usize;
    for i in 2..=num / 2 {
        if num % i == 0 {
            sum += i;
            factor.push(i.to_string());
        }
    }
    if sum == num {
        println!("{} = {}", num, factor.join(" + "));
        return true;
    }

    false
}
