use crate::util;
use std::collections::HashMap;

static REQUIRED_KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn run_easy() {
    let mut valid_count = 0;
    for passport in get_passport_data() {
        if valid_keys(&passport) {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
}

pub fn run_hard() {
    let mut valid_count = 0;
    for passport in get_passport_data() {
        if valid_keys(&passport) {
            let mut valid = true;
            valid &= passport.get("byr").filter(|val| val.len() == 4).and_then(|val| val.parse::<i32>().ok()).filter(|val| *val >= 1920 && *val <= 2002).is_some();
            valid &= passport.get("iyr").filter(|val| val.len() == 4).and_then(|val| val.parse::<i32>().ok()).filter(|val| *val >= 2010 && *val <= 2020).is_some();
            valid &= passport.get("eyr").filter(|val| val.len() == 4).and_then(|val| val.parse::<i32>().ok()).filter(|val| *val >= 2020 && *val <= 2030).is_some();
            let height = passport.get("hgt").unwrap();
            if height.ends_with("cm") {
                valid &= height[..height.len()-2].parse::<i32>().ok().filter(|n| (150..=193).contains(n)).is_some()
            } else if height.ends_with("in") {
                valid &= height[..height.len()-2].parse::<i32>().ok().filter(|n| (59..=76).contains(n)).is_some()
            } else {
                valid = false;
            }
            valid &= passport.get("hcl").map(|val| val.split_at(1)).filter(|val| (*val).0 == "#").filter(|val| (*val).1.len() == 6 && (*val).1.chars().all(|c| c >= '0' && c <= '9' || c >= 'a' && c <= 'f')).is_some();
            valid &= VALID_EYE_COLORS.contains(&&**passport.get("ecl").unwrap());
            valid &= passport.get("pid").filter(|val| val.len() == 9 && val.parse::<usize>().is_ok()).is_some();
            if valid {
                valid_count += 1;
            }
        }
    }
    println!("{}", valid_count);
}

fn valid_keys(passport: &HashMap<String, String>) -> bool {
    return REQUIRED_KEYS.iter().all(|key| passport.contains_key(&String::from(*key)));
}

fn get_passport_data() -> Vec<HashMap<String, String>> {
    return util::get_input_lines().collect::<Vec<_>>().as_slice()
        .split(|line| line.is_empty())
        .map(|passport_lines| passport_lines.iter()
            .flat_map(|line| line.split(" "))
            .map(|token| token.split_at(token.find(":").unwrap()))
            .map(|(a, b)| (String::from(a), String::from(b.strip_prefix(":").unwrap())))
            .collect())
        .collect();
}
