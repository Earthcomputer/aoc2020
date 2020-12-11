use crate::util;
use std::iter::FromIterator;

pub fn run_easy() {
    let seats = get_input();
    let occupied = get_seats_occupied(&seats, easy_mutator);
    println!("{}", occupied);
}

fn easy_mutator(x: usize, y: usize, seats_in: &Vec<Vec<char>>) -> char {
    let mut neighbors = 0;
    for dx in std::cmp::max(0, x as i32 - 1) as usize..=std::cmp::min(seats_in[0].len() - 1, x + 1) {
        for dy in std::cmp::max(0, y as i32 - 1) as usize..=std::cmp::min(seats_in.len() - 1, y + 1) {
            if dx != x || dy != y {
                if seats_in[dy][dx] == '#' {
                    neighbors += 1;
                }
            }
        }
    }
    return if seats_in[y][x] == 'L' && neighbors == 0 {
        '#'
    } else if seats_in[y][x] == '#' && neighbors >= 4 {
        'L'
    } else {
        seats_in[y][x]
    }
}

pub fn run_hard() {
    let seats = get_input();
    let occupied = get_seats_occupied(&seats, hard_mutator);
    println!("{}", occupied);
}

fn hard_mutator(x: usize, y: usize, seats_in: &Vec<Vec<char>>) -> char {
    let mut neighbors = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                let mut i = x as i32 + dx;
                let mut j = y as i32 + dy;
                while i >= 0 && j >= 0 && i < seats_in[0].len() as i32 && j < seats_in.len() as i32 {
                    let old_char = seats_in[j as usize][i as usize];
                    if old_char != '.' {
                        if old_char == '#' {
                            neighbors += 1;
                        }
                        break;
                    }
                    i += dx;
                    j += dy;
                }
            }
        }
    }
    return if seats_in[y][x] == 'L' && neighbors == 0 {
        '#'
    } else if seats_in[y][x] == '#' && neighbors >= 5 {
        'L'
    } else {
        seats_in[y][x]
    }
}

fn get_seats_occupied(seats: &Vec<Vec<char>>, mutator: fn(usize, usize, &Vec<Vec<char>>) -> char) -> u32 {
    let result_seats = simulate(seats, mutator);
    let mut occupied = 0;
    for line in result_seats {
        for seat in line {
            if seat == '#' {
                occupied += 1;
            }
        }
    }
    return occupied;
}

fn simulate(seats: &Vec<Vec<char>>, mutator: fn(usize, usize, &Vec<Vec<char>>) -> char) -> Vec<Vec<char>> {
    let mut seats1 = seats.clone();
    let mut seats2 = seats.clone();
    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..seats1.len() {
            for x in 0..seats1[0].len() {
                if seats1[y][x] != '.' {
                    let new_char = mutator(x, y, &seats1);
                    if new_char != seats1[y][x] {
                        seats2[y][x] = new_char;
                        changed = true;
                    }
                }
            }
        }

        seats1 = seats2.clone();
    }

    return seats1;
}

fn get_input() -> Vec<Vec<char>> {
    util::get_input_lines().map(|s| s.chars().collect()).collect()
}
