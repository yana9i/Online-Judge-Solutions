use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut space_index = input.find(' ').unwrap();

    let mut has_error = false;

    let a_i32 = match input[0..space_index].parse::<i32>() {
        Ok(i) => match i {
            1..=1000 => {
                print!("{} + ", i);
                i
            }
            _ => {
                has_error = true;
                print!("? + ");
                -1
            }
        },
        Err(error) => {
            has_error = true;
            print!("? + ");
            -1
        }
    };

    let b_i32 = match input.trim()[(space_index + 1)..].parse::<i32>() {
        Ok(i) => match i {
            1..=1000 => {
                print!("{} = ", i);
                i
            }
            _ => {
                has_error = true;
                print!("? = ");
                -1
            }
        },
        Err(error) => {
            has_error = true;
            print!("? = ");
            -1
        }
    };

    if has_error {
        println!("?");
    } else {
        println!("{}", a_i32 + b_i32)
    }
}
