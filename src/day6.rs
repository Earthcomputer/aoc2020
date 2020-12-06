use crate::util;

pub fn run_easy() {
    let mut count = 0;
    for group in get_input() {
        let mut yes = [0; 26];
        count_yeses(&group, &mut yes);
        count += yes.iter().filter(|val| **val != 0).count();
    }
    println!("{}", count);
}

pub fn run_hard() {
    let mut count = 0;
    for group in get_input() {
        let mut yes_count = [0; 26];
        count_yeses(&group, &mut yes_count);
        count += yes_count.iter().filter(|val| **val == group.len()).count();
    }
    println!("{}", count);
}

fn count_yeses(group: &Vec<String>, yes_count: &mut [usize; 26]) {
    for person in group {
        for question in person.chars().map(|c| c as usize) {
            if question >= 'a' as usize && question <= 'z' as usize {
                yes_count[question - 'a' as usize] += 1;
            }
        }
    }
}

fn get_input() -> Vec<Vec<String>> {
    return util::get_input_lines().collect::<Vec<_>>().as_slice()
        .split(|line| line.is_empty())
        .map(|group_lines| group_lines.iter().map(|s| String::from(s)).collect())
        .collect()
}
