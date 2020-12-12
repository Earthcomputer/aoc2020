use crate::util;
use std::iter::FromIterator;

pub fn run_easy() {
    let mut degrees = 0;
    let mut x = 0;
    let mut y = 0;
    for (insn, operand) in get_instructions() {
        match insn {
            'N' => y -= operand,
            'S' => y += operand,
            'W' => x -= operand,
            'E' => x += operand,
            'L' => degrees += operand,
            'R' => degrees -= operand,
            'F' => match degrees % 360 {
                0 => x += operand,
                90 | -270 => y -= operand,
                180 | -180 => x -= operand,
                270 | -90 => y += operand,
                _ => unreachable!()
            }
            _ => unreachable!()
        }
    }
    println!("{}", x.abs() + y.abs());
}

pub fn run_hard() {
    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = -1;
    for (insn, operand) in get_instructions() {
        match insn {
            'N' => waypoint_y -= operand,
            'S' => waypoint_y += operand,
            'W' => waypoint_x -= operand,
            'E' => waypoint_x += operand,
            'L' => {
                let new_x = waypoint_x * cos(operand) + waypoint_y * sin(operand);
                let new_y = waypoint_y * cos(operand) - waypoint_x * sin(operand);
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            'R' => {
                let new_x = waypoint_x * cos(operand) - waypoint_y * sin(operand);
                let new_y = waypoint_y * cos(operand) + waypoint_x * sin(operand);
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            'F' => {
                x += operand * waypoint_x;
                y += operand * waypoint_y;
            }
            _ => unreachable!()
        }
    }
    println!("{}", x.abs() + y.abs());
}

fn cos(n: i32) -> i32 {
    match n % 360 {
        90 | 270 | -90 | -270 => 0,
        0 => 1,
        180 | -180 => -1,
        _ => unreachable!()
    }
}

fn sin(n: i32) -> i32 {
    match n % 360 {
        0 | 180 | -180 => 0,
        90 | -270 => 1,
        270 | -90 => -1,
        _ => unreachable!()
    }
}

fn get_instructions() -> impl Iterator<Item = (char, i32)> {
    util::get_input_lines()
        .map(|line| (line.chars().nth(0).unwrap(), String::from_iter(line.chars().skip(1)).parse().unwrap()))
}
