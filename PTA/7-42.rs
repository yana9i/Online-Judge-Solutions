use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut line_tuple: Vec<(String, u32)> = vec![];
    let mut sum = 0_u32;
    for n in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<&str> = input.trim().split_whitespace().collect::<Vec<&str>>();
        let item = (line[0].to_string(), line[1].parse::<u32>().unwrap());
        sum += item.1;
        line_tuple.push(item);
    }
    let mut res_line = (String::new(), 0_u32);
    let half_avg = sum as f64 / n as f64 / 2.0;
    let mut differenc = f64::MAX;
    for i in line_tuple {
        let diff_line = (half_avg - i.1 as f64).abs();
        if diff_line < differenc {
            differenc = diff_line;
            res_line = i;
        }
    }
    println!("{:.0} {}", half_avg, res_line.0);
}
