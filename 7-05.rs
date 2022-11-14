use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut used_arr = [0; 100];
    let mut has_cap = false;

    for i in input.as_bytes() {
        if i >= &('A' as u8) && i <= &('Z' as u8) && used_arr[*i as usize] == 0 {
            print!("{}", *i as char);
            used_arr[*i as usize] = 1;
            has_cap = true;
        }
    }

    if has_cap {
        println!();
    } else {
        println!("Not Found");
    }
}
