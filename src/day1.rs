use crate::util;

pub fn run_easy() {
    let lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();
    for (ia, a) in lines.iter().enumerate() {
        for (ib, b) in lines.iter().take(ia).enumerate() {
            if ia != ib && a + b == 2020 {
                println!("{}", a * b);
            }
        }
    }
}

pub fn run_hard() {
    let lines: Vec<i32> = util::get_input_lines().map(|s| s.parse::<i32>().unwrap()).collect();
    for (ia, a) in lines.iter().enumerate() {
        for (ib, b) in lines.iter().take(ia).enumerate() {
            for (ic, c) in lines.iter().take(ib).enumerate() {
                if ia != ib && ib != ic && a + b + c == 2020 {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}