use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut n: u32 = input.trim().parse().unwrap();
    let arrSize = n as usize;

    struct addrList {
        name: String,
        birth: String,
        gender: String,
        linep: String,
        telep: String,
    }

    let mut add_col: Vec<addrList> = Vec::new();

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut s = input.trim().split_whitespace();
        add_col.push(addrList {
            name: s.next().unwrap().trim().to_string(),
            birth: s.next().unwrap().trim().to_string(),
            gender: s.next().unwrap().trim().to_string(),
            linep: s.next().unwrap().trim().to_string(),
            telep: s.next().unwrap().trim().to_string(),
        })
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut s = input.trim().split_whitespace();
    n = s.next().unwrap().trim().parse().unwrap();
    for i in 0..n {
        let li: usize = match s.next().unwrap().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not Found");
                continue;
            }
        };
        if li >= arrSize {
            println!("Not Found");
        } else {
            let arr = add_col.get(li).unwrap();
            println!(
                "{} {} {} {} {}",
                arr.name, arr.linep, arr.telep, arr.gender, arr.birth
            );
        }
    }
}
