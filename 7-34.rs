use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    let mut cube_arr: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    cube_arr = cube_arr.map(|x| x.pow(n));
    let start_i = 10_u32.pow(n - 1);
    for i in start_i..(start_i * 10) {
        let i_str = i.to_string().clone();
        let i_bytes = i_str.as_bytes();
        let mut sum = 0_u32;
        for &j in i_bytes {
            sum += cube_arr[(j - b'0') as usize];
        }
        // println!("{} sum {}", i, sum);
        if i == sum {
            println!("{}", i);
        }
    }
}
