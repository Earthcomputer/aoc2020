mod util;
mod day1;

fn main() {
    let day = util::get_filtered_int_input("What day do you want to solve?", |i| i > 0 && i < 25);
    let hard = util::get_filtered_int_input("Which part of the day do you want to solve?", |i| i == 1 || i == 2) == 2;
    match day {
        1 => if hard {
            day1::run_hard()
        } else {
            day1::run_easy()
        },
        _ => panic!("Day {} not implemented", day)
    }
}
