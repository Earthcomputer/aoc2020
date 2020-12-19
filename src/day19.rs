use crate::util;
use std::collections::{HashMap, HashSet, BTreeMap};

pub fn run_easy() {
    let input = get_input();
    let matches = input.strings.iter().filter(|s| match_full_input(&input, &s.chars().collect())).count();
    println!("{}", matches);
}

pub fn run_hard() {
    let mut input = get_input();
    input.expansions.insert(8, vec![vec![42], vec![42, 8]]);
    input.expansions.insert(11, vec![vec![42, 31], vec![42, 11, 31]]);
    let matches = input.strings.iter().filter(|s| match_full_input(&input, &s.chars().collect())).count();
    println!("{}", matches);
}

fn match_full_input(rules: &Input, input: &Vec<char>) -> bool {
    match_input(rules, input, 0, 0).contains(&input.len())
}

fn match_input(rules: &Input, input: &Vec<char>, rule: u32, index: usize) -> HashSet<usize> {
    let mut result = HashSet::new();
    if rules.letters.contains_key(&rule) {
        if index < input.len() && input[index] == *rules.letters.get(&rule).unwrap() {
            result.insert(index + 1);
        }
        return result;
    }

    let expansions = rules.expansions.get(&rule).unwrap();
    for expansion in expansions {
        let mut end_indexes = BTreeMap::new();
        let mut initial_set = HashSet::new();
        initial_set.insert(0);
        end_indexes.insert(index, initial_set);
        while !end_indexes.is_empty() {
            let (sub_index_ref, rule_indexes_ref) = end_indexes.iter().next().unwrap();
            let sub_index = *sub_index_ref;
            let rule_indexes = rule_indexes_ref.clone();
            end_indexes.remove(&sub_index);
            for rule_index in rule_indexes {
                let sub_end_indexes = match_input(rules, input, expansion[rule_index], sub_index);
                if rule_index == expansion.len() - 1 {
                    for sub_index in sub_end_indexes {
                        result.insert(sub_index);
                    }
                } else {
                    for sub_index in sub_end_indexes {
                        end_indexes.entry(sub_index).or_insert_with(|| HashSet::new()).insert(rule_index + 1);
                    }
                }
            }
        }
    }

    return result;
}

fn get_input() -> Input {
    let lines: Vec<_> = util::get_input_lines().collect();
    let blocks: Vec<_> = lines.as_slice().split(|s| s.is_empty()).collect();
    let mut letters = HashMap::new();
    let mut expansions = HashMap::new();
    for line in blocks[0] {
        let (num_str, mut rest) = line.split_at(line.find(":").unwrap());
        let num = num_str.parse().unwrap();
        rest = rest.strip_prefix(": ").unwrap();
        if rest.starts_with("\"") {
            let letter = rest.chars().nth(1).unwrap();
            letters.insert(num, letter);
        } else {
            let expansion = rest.split(" | ")
                .map(|s| s.split(" ")
                    .map(|s| s.parse().unwrap())
                    .collect())
                .collect();
            expansions.insert(num, expansion);
        }
    }

    let strings = Vec::from(blocks[1]);

    return Input {
        letters, expansions, strings
    }
}

struct Input {
    letters: HashMap<u32, char>,
    expansions: HashMap<u32, Vec<Vec<u32>>>,
    strings: Vec<String>
}
