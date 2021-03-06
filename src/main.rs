mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

fn main() {
    let day = util::get_filtered_int_input("What day do you want to solve?", |i| i > 0 && i < 25);
    let hard = util::get_filtered_int_input("Which part of the day do you want to solve?", |i| i == 1 || i == 2) == 2;
    match day {
        1 => if hard {
            day1::run_hard()
        } else {
            day1::run_easy()
        },
        2 => if hard {
            day2::run_hard()
        } else {
            day2::run_easy()
        },
        3 => if hard {
            day3::run_hard()
        } else {
            day3::run_easy()
        },
        4 => if hard {
            day4::run_hard()
        } else {
            day4::run_easy()
        },
        5 => if hard {
            day5::run_hard()
        } else {
            day5::run_easy()
        },
        6 => if hard {
            day6::run_hard()
        } else {
            day6::run_easy()
        },
        7 => if hard {
            day7::run_hard()
        } else {
            day7::run_easy()
        },
        8 => if hard {
            day8::run_hard()
        } else {
            day8::run_easy()
        },
        9 => if hard {
            day9::run_hard()
        } else {
            day9::run_easy()
        },
        10 => if hard {
            day10::run_hard()
        } else {
            day10::run_easy()
        },
        11 => if hard {
            day11::run_hard()
        } else {
            day11::run_easy()
        },
        12 => if hard {
            day12::run_hard()
        } else {
            day12::run_easy()
        },
        13 => if hard {
            day13::run_hard()
        } else {
            day13::run_easy()
        },
        14 => if hard {
            day14::run_hard()
        } else {
            day14::run_easy()
        },
        15 => if hard {
            day15::run_hard()
        } else {
            day15::run_easy()
        },
        16 => if hard {
            day16::run_hard()
        } else {
            day16::run_easy()
        },
        17 => if hard {
            day17::run_hard()
        } else {
            day17::run_easy()
        },
        18 => if hard {
            day18::run_hard()
        } else {
            day18::run_easy()
        },
        19 => if hard {
            day19::run_hard()
        } else {
            day19::run_easy()
        },
        20 => if hard {
            day20::run_hard()
        } else {
            day20::run_easy()
        },
        21 => if hard {
            day21::run_hard()
        } else {
            day21::run_easy()
        },
        22 => if hard {
            day22::run_hard()
        } else {
            day22::run_easy()
        }
        _ => panic!("Day {} not implemented", day)
    }
}
