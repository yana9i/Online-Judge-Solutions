use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_backup: u32 = input.trim().parse().unwrap();

    let n_sqrt = (n_backup as f64).sqrt() as u32;

    let mut fact_start = 0;
    let mut fact_count = 0;

    for i in 2..=n_sqrt {
        let mut n = n_backup;
        let mut count = 0;
        let mut j = i;
        while n % j == 0 {
            n /= j;
            j += 1;
            count += 1;
        }
        if count > fact_count {
            fact_count = count;
            fact_start = i;
        }
    }

    if fact_count > 0 {
        println!("{}", fact_count);
        for i in fact_start..fact_start + fact_count - 1 {
            print!("{}*", i);
        }
        println!("{}", fact_start + fact_count - 1);
    } else {
        println!("1\n{}", n_backup);
    }
}
