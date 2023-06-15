use std::io;

fn main() {
    let mut counter_arr: [u32; 26] = [0; 26];

    let mut max_count = 0;

    for i in 0..4 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for j in input.trim().chars() {
            if 'A' <= j && j <= 'Z' {
                let index = (j as u8 - b'A') as usize;
                counter_arr[index] += 1;
                if counter_arr[index] > max_count {
                    max_count = counter_arr[index];
                }
            }
        }
    }

    for i in (0..max_count).rev() {
        for j in 0..26 {
            if counter_arr[j] > i {
                print!("* ");
            } else {
                print!("  ")
            }
        }
        println!();
    }
    println!("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
}
