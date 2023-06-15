use std::{
    fmt::{self, format},
    io, string, vec,
};

fn main() {
    let mut res_vec: Vec<String> = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        }
        let mut s = input.trim().split_ascii_whitespace();
        let mut num = ArbPreNum::new(s.next().unwrap().to_string());
        let pow = s.next().unwrap().parse::<usize>().unwrap();
        let str = num.pow(pow).to_string();
        //println!("{}", str);
        res_vec.push(str);
    }
    print!("{}", res_vec.join("\n"));
}
#[derive(Clone)]
struct ArbPreNum {
    integer: String,
    decima: String,
}

impl ArbPreNum {
    pub fn new(string_val: String) -> ArbPreNum {
        let dot_index = match string_val.find('.') {
            Some(s) => s,
            None => 0_usize,
        };
        let mut integer = string_val[0..dot_index].trim_start_matches('0').to_string();
        let mut decima = string_val[dot_index + 1..]
            .trim_end_matches('0')
            .to_string();
        ArbPreNum { integer, decima }
    }

    pub fn mult(&mut self, other_apn: &ArbPreNum) -> &ArbPreNum {
        let a_full_str = self.full_str();
        let b_full_str = other_apn.full_str();
        let mut res_str = str_mult_str(a_full_str, b_full_str);
        let decima_digits = self.decima.len() + other_apn.decima.len();
        if decima_digits > res_str.len() {
            while res_str.len() < decima_digits {
                res_str.insert(0, '0');
            }
            self.integer = "0".to_string();
            self.decima = res_str.trim_end_matches('0').to_string();
        } else {
            self.integer = res_str[0..res_str.len() - decima_digits].to_string();
            self.decima = res_str[res_str.len() - decima_digits..]
                .trim_end_matches('0')
                .to_string();
        }

        self
    }

    pub fn pow(&mut self, exp: usize) -> &ArbPreNum {
        let mut temp_num = self.clone();
        for i in 0..exp - 1 {
            self.mult(&mut temp_num);
        }
        self
    }

    pub fn full_str(&self) -> String {
        let mut res = format!("{}{}", self.integer, self.decima);
        res.trim_start_matches('0');
        res
    }
}

impl fmt::Display for ArbPreNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut integer = self.integer.trim_start_matches('0').to_string();
        let mut decima = self.decima.trim_end_matches('0').to_string();
        let mut dot = String::new();
        if !decima.is_empty() {
            dot = ".".to_string();
        }
        if integer.is_empty() {
            integer = "".to_string();
        }
        write!(f, "{}", format!("{}{}{}", integer, dot, decima))
    }
}

fn str_mult_str(a: String, b: String) -> String {
    if a.eq(&"0".to_string()) || b.eq(&"0".to_string()) || a.is_empty() || b.is_empty() {
        return "0".to_string();
    }
    let a_char = a.chars().rev().collect::<Vec<char>>();
    let b_char = b.chars().rev().collect::<Vec<char>>();
    let mut res_str = String::new();

    for i in 0..a_char.len() {
        let a_num = a_char[i].to_digit(10).unwrap();
        let mut carry = 0_u32;
        let mut mult_1_num = String::new();
        for j in 0..b_char.len() {
            let b_num = b_char[j].to_digit(10).unwrap();
            let res = a_num * b_num + carry;
            carry = res / 10;
            let module = res % 10;
            mult_1_num.push((module as u8 + b'0') as char);
        }
        if carry > 0 {
            mult_1_num.push((carry as u8 + b'0') as char);
        }
        mult_1_num = mult_1_num.chars().rev().collect::<String>();
        for j in 0..i {
            mult_1_num.push('0');
        }
        res_str = str_plus_str(res_str, mult_1_num);
    }

    res_str
}

fn str_plus_str(a: String, b: String) -> String {
    let a_char = a.chars().rev().collect::<Vec<char>>();
    let b_char = b.chars().rev().collect::<Vec<char>>();
    let mut res_str = String::new();
    let max_len = a_char.len().max(b_char.len());
    let mut carry = 0_u32;

    for i in 0..max_len {
        let mut a = 0_u32;
        let mut b = 0_u32;
        if a_char.len() > i {
            a = a_char[i].to_digit(10).unwrap();
        }
        if b_char.len() > i {
            b = b_char[i].to_digit(10).unwrap();
        }
        let res = a + b + carry;
        carry = res / 10;
        let module = res % 10;
        res_str.push((module as u8 + b'0') as char);
    }

    if carry > 0 {
        res_str.push((carry as u8 + b'0') as char);
    }

    res_str.chars().rev().collect::<String>()
}
