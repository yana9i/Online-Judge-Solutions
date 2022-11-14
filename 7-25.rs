use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let str = input.trim().to_owned().clone();

    let offset: i32 = 1;

    for i in str.chars() {
        match i {
            'a'..='z' => {
                let mut j = (i as i32 + offset - 'a' as i32) % 26;
                if j < 0 {
                    j += 26;
                }
                print!("{}", ((j as u8 + b'a') as char).to_uppercase());
            }
            'A'..='Z' => {
                let mut j = (i as i32 + offset - 'A' as i32) % 26;
                if j < 0 {
                    j += 26;
                }
                print!("{}", ((j as u8 + b'A') as char).to_lowercase());
            }
            _ => {
                print!("{}", i);
            }
        }
    }
}