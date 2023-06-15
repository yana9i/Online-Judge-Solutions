use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut martrix: Vec<Vec<bool>> = vec![];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<bool> = input.trim().chars().map(|x| x == 'y').collect();
        martrix.push(line);
    }
    let mut min_counter = u32::MAX;
    for i in 0..2_usize.pow(n as u32) {
        let mut counter = 0_u32;
        let mut martrix_clone = martrix.clone();
        let bit_iter_str = format!("{:0width$b}", i, width = n);
        for (i, &item) in bit_iter_str.as_bytes().iter().enumerate() {
            if item == b'1' {
                flip_martrix(0, i, &mut martrix_clone);
                counter += 1;
            }
        }
        for i in 0..n - 1 {
            for j in 0..n {
                if !martrix_clone[i][j] {
                    flip_martrix(i + 1, j, &mut martrix_clone);
                    counter += 1;
                }
            }
        }

        let mut is_all_yellow = true;
        for &i in &martrix_clone[n - 1] {
            if !i {
                is_all_yellow = false;
            }
        }

        if is_all_yellow && counter < min_counter {
            min_counter = counter;
        }
    }
    if min_counter == u32::MAX {
        println!("inf");
    } else {
        println!("{}", min_counter);
    }
}

fn flip_martrix(line: usize, index: usize, martrix: &mut Vec<Vec<bool>>) {
    martrix[line][index] = !martrix[line][index];
    if index > 0 {
        martrix[line][index - 1] = !martrix[line][index - 1];
    }
    if index < martrix[0].len() - 1 {
        martrix[line][index + 1] = !martrix[line][index + 1];
    }
    if line < martrix.len() - 1 {
        martrix[line + 1][index] = !martrix[line + 1][index];
    }
    if line > 0 {
        martrix[line - 1][index] = !martrix[line - 1][index];
    }
}
