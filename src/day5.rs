use crate::util;
use std::cmp::max;
use std::collections::HashSet;

pub fn run_easy() {
    let mut max_id = 0;
    for (row, col) in read_input() {
        max_id = max(max_id, get_id(row, col));
        continue;
    }
    println!("{}", max_id);
}

pub fn run_hard() {
    let seats: HashSet<_> = read_input().map(|val| get_id(val.0, val.1)).collect();
    let min_id = seats.iter().min().unwrap();
    let max_id = seats.iter().max().unwrap();
    for id in *min_id..=*max_id {
        if !seats.contains(&id) {
            println!("{}", id);
        }
    }
}

fn get_id(row: i32, col: i32) -> i32 {
    return row * 8 + col;
}

fn read_input() -> impl Iterator<Item = (i32, i32)> {
    return util::get_input_lines()
        .map(|line| line.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1"))
        .map(|val| i32::from_str_radix(&*val, 2).unwrap())
        .map(|val| (val >> 3, val & 7));
}
