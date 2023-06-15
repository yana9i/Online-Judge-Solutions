use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();

    let mut max = i32::MIN;
    let mut max_name = String::new();

    for i in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line_vec: Vec<String> = line.trim().split(" ").map(|x| x.to_string()).collect();
        let line_num: i32 = line_vec[2].parse::<i32>().unwrap()
            + line_vec[3].parse::<i32>().unwrap()
            + line_vec[4].parse::<i32>().unwrap();
        if line_num > max {
            max = line_num;
            max_name = format!("{} {} {}", line_vec[1], line_vec[0], line_num);
        }
    }

    println!("{}", max_name);
}