use crate::util;
use std::iter::FromIterator;
use std::collections::HashSet;

pub fn run_easy() {
    let seats: HashSet<_> = get_input().iter().map(|(x, y)| (*x, *y, 0)).collect();
    let occupied = simulate3d(&seats, 6).len();
    println!("{}", occupied);
}

fn get_neighbors3d((ox, oy, oz): (isize, isize, isize)) -> impl Iterator<Item = (isize, isize, isize)> {
    (-1..=1).flat_map(move |x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (ox + x, oy + y, oz + z))))
}

fn simulate3d(seats: &HashSet<(isize, isize, isize)>, rounds: usize) -> HashSet<(isize, isize, isize)> {
    let mut seats1 = seats.clone();
    let mut seats2 = seats.clone();
    let mut round_count = 0;
    while round_count < rounds {
        for (x, y, z) in seats1.iter().flat_map(|pos| get_neighbors3d(*pos)).collect::<HashSet<_>>() {
            let mut neighbors = 0;
            for dx in x-1..=x+1 {
                for dy in y-1..=y+1 {
                    for dz in z-1..=z+1 {
                        if dx != x || dy != y || dz != z {
                            if seats1.contains(&(dx, dy, dz)) {
                                neighbors += 1;
                            }
                        }
                    }
                }
            }
            let was_active = seats1.contains(&(x, y, z));
            let new_active = if was_active && (neighbors != 2 && neighbors != 3) {
                false
            } else if !was_active && neighbors == 3 {
                true
            } else {
                was_active
            };
            if new_active {
                seats2.insert((x, y, z));
            } else {
                seats2.remove(&(x, y, z));
            }
        }
        seats1 = seats2.clone();
        round_count += 1;
    }

    return seats1;
}

pub fn run_hard() {
    let seats: HashSet<_> = get_input().iter().map(|(x, y)| (*x, *y, 0, 0)).collect();
    let occupied = simulate4d(&seats, 6).len();
    println!("{}", occupied);
}

fn get_neighbors4d((ox, oy, oz, ow): (isize, isize, isize, isize)) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    (-1..=1).flat_map(move |x| (-1..=1).flat_map(move |y| (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (ox + x, oy + y, oz + z, ow + w)))))
}

fn simulate4d(seats: &HashSet<(isize, isize, isize, isize)>, rounds: usize) -> HashSet<(isize, isize, isize, isize)> {
    let mut seats1 = seats.clone();
    let mut seats2 = seats.clone();
    let mut round_count = 0;
    while round_count < rounds {
        for (x, y, z, w) in seats1.iter().flat_map(|pos| get_neighbors4d(*pos)).collect::<HashSet<_>>() {
            let mut neighbors = 0;
            for dx in x-1..=x+1 {
                for dy in y-1..=y+1 {
                    for dz in z-1..=z+1 {
                        for dw in w-1..=w+1 {
                            if dx != x || dy != y || dz != z || dw != w {
                                if seats1.contains(&(dx, dy, dz, dw)) {
                                    neighbors += 1;
                                }
                            }
                        }
                    }
                }
            }
            let was_active = seats1.contains(&(x, y, z, w));
            let new_active = if was_active && (neighbors != 2 && neighbors != 3) {
                false
            } else if !was_active && neighbors == 3 {
                true
            } else {
                was_active
            };
            if new_active {
                seats2.insert((x, y, z, w));
            } else {
                seats2.remove(&(x, y, z, w));
            }
        }
        seats1 = seats2.clone();
        round_count += 1;
    }

    return seats1;
}

fn get_input() -> Vec<(isize, isize)> {
    util::get_input_lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .enumerate()
        .flat_map(|(y, line)| line.iter()
            .enumerate()
            .filter(|(_, v)| **v == '#')
            .map(move |(x, _)| (x as isize, y as isize))
            .collect::<Vec<_>>())
        .collect()
}
