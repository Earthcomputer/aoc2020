use crate::util;
use std::collections::{HashMap, HashSet};

pub fn run_easy() {
    let (names, by_name, successors) = parse_input();
    let mut can_be_contained_in: Vec<HashSet<usize>> = names.iter().map(|_| HashSet::new()).collect();
    for i in 0..names.len() {
        fill_can_contain(i, i, &mut can_be_contained_in, &successors);
    }
    println!("{}", can_be_contained_in[*by_name.get("shiny gold").unwrap()].len())
}

fn fill_can_contain(color: usize, index: usize, can_be_contained_in: &mut Vec<HashSet<usize>>, successors: &Vec<Vec<(usize, usize)>>) {
    for (_, succ) in &successors[index] {
        can_be_contained_in[*succ].insert(color);
        fill_can_contain(color, *succ, can_be_contained_in, successors);
    }
}

pub fn run_hard() {
    let (_, by_name, successors) = parse_input();
    println!("{}", count_recursive(*by_name.get("shiny gold").unwrap(), &successors) - 1);
}

fn count_recursive(index: usize, successors: &Vec<Vec<(usize, usize)>>) -> usize {
    let mut count = 0;
    for (num, succ) in &successors[index] {
        count += num * count_recursive(*succ, successors);
    }
    return count + 1;
}

fn parse_input() -> (Vec<String>, HashMap<String, usize>, Vec<Vec<(usize, usize)>>) {
    let mut names = Vec::new();
    let mut by_name = HashMap::new();
    let mut str_successors = Vec::new();
    for (index, line) in util::get_input_lines().enumerate() {
        let (color, mut rest) = line.split_at(line.find(" bags contain ").unwrap());
        rest = rest.strip_prefix(" bags contain ").unwrap();
        rest = rest.strip_suffix(".").unwrap();
        names.push(String::from(color));
        by_name.insert(String::from(color), index);
        if rest == "no other bags" {
            str_successors.push(Vec::new());
        } else {
            let parts = rest.split(",");
            let mut succ = Vec::new();
            for mut part in parts {
                part = part.strip_prefix(" ").unwrap_or(part);
                part = match part.strip_suffix(" bags") {
                    Some(val) => val,
                    None => part.strip_suffix(" bag").unwrap()
                };
                let (count, rest) = part.split_at(part.find(" ").unwrap());
                succ.push((count.parse::<usize>().unwrap(), String::from(rest.strip_prefix(" ").unwrap())));
            }
            str_successors.push(succ);
        }
    }
    let successors = str_successors.iter().map(|vec| vec.iter().map(|(k, v)| (*k, *by_name.get(v).unwrap())).collect()).collect();
    return (names, by_name, successors);
}
