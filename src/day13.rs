use crate::util;

pub fn run_easy() {
    let (earliest_time, ids) = get_input();
    let mut min_time = usize::MAX;
    let mut min_id = 0;
    for id in ids {
        if id == 0 {
            continue;
        }
        let mut k = earliest_time % id;
        k = id - k;
        if k == id {
            k = 0;
        }
        if k < min_time {
            min_time = k;
            min_id = id;
        }
    }
    println!("{}", min_time * min_id);
}

pub fn run_hard() {
    let (_, ids) = get_input();
    let mut product: u64 = 1;
    let mut result: u64 = 0;
    for (index, id) in ids.iter().enumerate() {
        let mut value = index as u64;
        let modulus = *id as u64;
        if modulus == 0 {
            continue;
        }
        value %= modulus;
        value = modulus - value;
        if value == modulus {
            value = 0;
        }
        while result % modulus != value {
            result += product;
        }
        product *= modulus;
    }
    println!("{}", result);
}

fn get_input() -> (usize, Vec<usize>) {
    let lines: Vec<_> = util::get_input_lines().collect();
    let earliest_time = lines[0].parse().unwrap();
    let ids = lines[1].split(",").map(|s| if s == "x" {
        0
    } else {
        s.parse().unwrap()
    }).collect();
    return (earliest_time, ids);
}
