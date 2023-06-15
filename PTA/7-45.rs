use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<u32>().unwrap();
    input.clear();

    for i in 0..t {
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<u32>().unwrap();
        input.clear();
        let mut is_upper_tri = true;

        for n in 0..n {
            io::stdin().read_line(&mut input).unwrap();
            let line_num = input
                .trim()
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            input.clear();
            for &item in &line_num[0..n as usize] {
                if item != 0 {
                    is_upper_tri = false;
                }
            }
        }
        if is_upper_tri {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
