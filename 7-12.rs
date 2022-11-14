use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut is_new_word = true;

    for i in input.chars() {
        if i == ' ' {
            is_new_word = true;
            print!(" ");
        } else {
            if is_new_word && 'a' <= i && i <= 'z' {
                print!("{}", i.to_uppercase());
            } else {
                print!("{}", i);
            }
            is_new_word = false;
        }
    }
}
