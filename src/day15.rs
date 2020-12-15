use crate::util;
use std::collections::HashMap;

pub fn run_easy() {
    print_nth_number(2020);
}

pub fn run_hard() {
    print_nth_number(30000000);
}

fn print_nth_number(n: u32) {
    let mut turns: u32 = 0;
    let mut last_said = HashMap::new();
    let mut prev_number = 0;
    for i in get_input() {
        if turns != 0 {
            last_said.insert(prev_number, turns);
        }
        prev_number = i;
        turns += 1;
    }

    while turns < n {
        let next_number = match last_said.get(&prev_number) {
            Some(turn) => turns - turn,
            None => 0
        };
        last_said.insert(prev_number, turns);
        prev_number = next_number;
        turns += 1;
    }

    println!("{}", prev_number);
}

fn get_input() -> Vec<u32> {
    util::get_input_lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect()
}
