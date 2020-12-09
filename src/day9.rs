use crate::util;

const LOOK_BEHIND: usize = 25;

pub fn run_easy() {
    let numbers = read_input();
    let index = find_invalid_index(&numbers);
    println!("{}", numbers[index]);
}

fn find_invalid_index(numbers: &Vec<usize>) -> usize {
    let mut sums = [0; (LOOK_BEHIND - 1) * (LOOK_BEHIND - 1)];
    let mut index = 0;
    for i in 1..LOOK_BEHIND {
        for j in 1..LOOK_BEHIND {
            if i != j {
                sums[index] = numbers[i] + numbers[j];
                index += 1;
            }
        }
    }
    for i in LOOK_BEHIND..numbers.len() {
        if !sums.contains(&numbers[i]) {
            return i;
        }
        for behind in 1..LOOK_BEHIND {
            sums[(i - 1) % (LOOK_BEHIND - 1) * (LOOK_BEHIND - 1) + (behind - 1)] = numbers[i - behind] + numbers[i];
        }
    }
    unreachable!();
}

pub fn run_hard() {
    let numbers = read_input();
    let invalid_index = find_invalid_index(&numbers);
    let target_sum = numbers[invalid_index];
    'outer: for i in 0..invalid_index {
        let mut sum = 0;
        let mut min = usize::MAX;
        let mut max = 0;
        for j in i..invalid_index {
            sum += numbers[j];
            if numbers[j] < min {
                min = numbers[j];
            }
            if numbers[j] > max {
                max = numbers[j];
            }
            if sum == target_sum {
                println!("{}", min + max);
                break 'outer;
            }
        }
    }
}

pub fn read_input() -> Vec<usize> {
    return util::get_input_lines().map(|s| s.parse().unwrap()).collect();
}
