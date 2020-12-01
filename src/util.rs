use std::io;
use std::io::{Write, BufRead, BufReader};
use std::fs::File;

pub fn get_input(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().expect("error: failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let newlines: &[_] = &['\r', '\n'];
    input = String::from(input.trim_end_matches(newlines));
    return input;
}

pub fn get_int_input(prompt: &str) -> i32 {
    return get_filtered_int_input(prompt, |_| true);
}

pub fn get_filtered_int_input<F>(prompt: &str, predicate: F) -> i32
    where F: Fn(i32) -> bool {
    let answer = get_input(prompt);
    let result = answer.parse::<i32>();
    if result.is_ok() {
        let ret = result.unwrap();
        if predicate(ret) {
            return ret;
        }
    }

    loop {
        let answer = get_input("Invalid integer, try again:");
        let result = answer.parse::<i32>();
        if result.is_ok() {
            let ret = result.unwrap();
            if predicate(ret) {
                return ret;
            }
        }
    }
}

pub fn get_input_lines() -> impl Iterator<Item = String> {
    let file = File::open("input.txt").unwrap();
    return BufReader::new(file).lines().map(|line| line.unwrap());
}