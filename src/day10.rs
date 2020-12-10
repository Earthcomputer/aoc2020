use crate::util;

pub fn run_easy() {
    let vec = get_input();
    let mut ones = if vec[0] == 1 {
        1
    } else {
        0
    };
    let mut threes = if vec[0] == 3 {
        2
    } else {
        1
    };
    for i in 1..vec.len() {
        if vec[i] == vec[i - 1] + 1 {
            ones += 1;
        } else if vec[i] == vec[i - 1] + 3 {
            threes += 1;
        }
    }
    println!("{}", ones * threes);
}

pub fn run_hard() {
    let vec = get_input();
    let mut n_ways: Vec<usize> = vec![0; vec.len()];
    n_ways[0] = 1;
    if vec.len() >= 2 && vec[1] <= 3 {
        n_ways[1] = 1;
    }
    if vec.len() >= 3 && vec[2] <= 3 {
        n_ways[2] = 1;
    }
    for i in 0..(vec.len() - 1) {
        n_ways[i + 1] += n_ways[i];
        if i + 2 < vec.len() && vec[i + 2] <= vec[i] + 3 {
            n_ways[i + 2] += n_ways[i];
        }
        if i + 3 < vec.len() && vec[i + 3] <= vec[i] + 3 {
            n_ways[i + 3] += n_ways[i];
        }
    }
    println!("{}", n_ways.last().unwrap());
}

fn get_input() -> Vec<usize> {
    let mut vec: Vec<_> = util::get_input_lines().map(|s| s.parse().unwrap()).collect();
    vec.sort();
    return vec;
}
