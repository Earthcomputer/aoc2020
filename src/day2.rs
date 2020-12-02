use crate::util;

pub fn run_easy() {
    let mut valid_passwords = 0;

    for policy in util::get_input_lines().map(parse_password_policy) {
        let mut count = 0;
        for c in policy.password.chars() {
            if c == policy.letter {
                count += 1;
            }
        }

        if count >= policy.n1 && count <= policy.n2 {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}

pub fn run_hard() {
    let mut valid_passwords = 0;

    for policy in util::get_input_lines().map(parse_password_policy) {
        if [policy.n1, policy.n2].iter()
            .filter_map(|n| policy.password.chars().nth((n - 1) as usize))
            .filter(|letter| *letter == policy.letter)
            .count() == 1 {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}

fn parse_password_policy(str: String) -> PasswordWithPolicy {
    let colon_index = str.find(":").unwrap();
    let (before_colon, after_colon) = str.split_at(colon_index);
    let space_index = before_colon.find(" ").unwrap();
    let (before_space, after_space) = before_colon.split_at(space_index);
    let dash_index = before_space.find("-").unwrap();
    let (before_dash, after_dash) = before_space.split_at(dash_index);
    let password = after_colon.strip_prefix(": ").unwrap();
    return PasswordWithPolicy {
        letter: after_space.chars().nth(1).unwrap(),
        n1: before_dash.parse().unwrap(),
        n2: after_dash.strip_prefix("-").unwrap().parse().unwrap(),
        password: String::from(password)
    };
}

struct PasswordWithPolicy {
    letter: char,
    n1: i32,
    n2: i32,
    password: String
}