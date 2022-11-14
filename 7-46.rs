use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim_matches('\n').chars();
    let mut ascii_arr = [false; 128];
    for i in n {
        ascii_arr[i as usize] = true;
    }
    for (i, item) in ascii_arr.iter().enumerate() {
        if *item {
            print!("{}", i as u8 as char);
        }
    }
    println!()
}
