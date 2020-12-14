use crate::util;
use std::collections::HashMap;

const MASK: &str = "mask";

pub fn run_easy() {
    let mut zero_mask = 0;
    let mut one_mask = 0;
    let mut mem = Vec::new();
    for (opcode, operand1, operand2) in get_input() {
        if opcode == MASK {
            zero_mask = operand1;
            one_mask = operand2;
        } else {
            if operand1 >= mem.len() as u64 {
                mem.resize((operand1 + 1) as usize, 0);
            }
            mem[operand1 as usize] = (operand2 & zero_mask) | one_mask;
        }
    }

    println!("{}", mem.iter().sum::<u64>());
}

pub fn run_hard() {
    let mut zero_mask = 0;
    let mut one_mask = 0;
    let mut mem = HashMap::new();
    for (opcode, operand1, operand2) in get_input() {
        if opcode == MASK {
            zero_mask = operand1;
            one_mask = operand2;
        } else {
            let mut addresses = vec![(operand1 & !zero_mask) | one_mask];
            for i in 0..36 {
                let mask = 1 << i;
                if (zero_mask & mask) != (one_mask & mask) {
                    let len = addresses.len();
                    for j in 0..len {
                        addresses.push(addresses[j] | mask);
                    }
                }
            }
            for address in addresses {
                mem.insert(address, operand2);
            }
        }
    }

    println!("{}", mem.values().sum::<u64>());
}

fn get_input() -> impl Iterator<Item = (String, u64, u64)> {
    util::get_input_lines().map(|s| {
        let (mut a, mut b) = s.split_at(s.find(" = ").unwrap());
        b = b.strip_prefix(" = ").unwrap();
        let operand1 = match a.find("[") {
            Some(bracket_index) => {
                let (opcode, rest) = a.split_at(bracket_index);
                a = opcode;
                rest.strip_prefix("[").unwrap().strip_suffix("]").unwrap().parse().unwrap()
            },
            None => b.chars().enumerate().filter(|(_, v)| *v != '0').map(|(i, _)| 1 << (35 - i)).sum()
        };
        let operand2 = if a == MASK {
            b.chars().enumerate().filter(|(_, v)| *v == '1').map(|(i, _)| 1 << (35 - i)).sum()
        } else {
            b.parse().unwrap()
        };
        (String::from(a), operand1, operand2)
    })
}
