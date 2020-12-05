mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
        }
        _ => panic!("Day {} not implemented", day)
    }
}
