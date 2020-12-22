use crate::util;
use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;

pub fn run_easy() {
    let (mut player1, mut player2) = get_input();
    while !player1.is_empty() && !player2.is_empty() {
        let a = player1.pop_front().unwrap();
        let b = player2.pop_front().unwrap();
        if a > b {
            player1.push_back(a);
            player1.push_back(b);
        } else {
            player2.push_back(b);
            player2.push_back(a);
        }
    }

    let winner = if player1.is_empty() {
        player2
    } else {
        player1
    };

    let score = get_score(&winner);
    println!("{}", score);
}

pub fn run_hard() {
    let (player1, player2) = get_input();
    let (_, winner) = recursive_combat(player1, player2);
    let score = get_score(&winner);
    println!("{}", score);
}

fn recursive_combat(mut player1: VecDeque<i32>, mut player2: VecDeque<i32>) -> (i8, VecDeque<i32>) {
    let mut prev_rounds = HashSet::new();
    while !player1.is_empty() && !player2.is_empty() {
        if prev_rounds.contains(&(player1.clone(), player2.clone())) {
            return (1, player1.clone());
        }
        prev_rounds.insert((player1.clone(), player2.clone()));

        let a = player1.pop_front().unwrap();
        let b = player2.pop_front().unwrap();
        let (winner, _) = if player1.len() as i32 >= a && player2.len() as i32 >= b {
            recursive_combat(player1.iter().take(a as usize).map(|card| *card).collect(),
                             player2.iter().take(b as usize).map(|card| *card).collect())
        } else if a > b {
            (1, player1.clone())
        } else {
            (2, player2.clone())
        };

        if winner == 1 {
            player1.push_back(a);
            player1.push_back(b);
        } else {
            player2.push_back(b);
            player2.push_back(a);
        }
    }
    return if player1.is_empty() {
        (2, player2)
    } else {
        (1, player1)
    };
}

fn get_score(winner: &VecDeque<i32>) -> u64 {
    winner.iter().rev().enumerate().map(|(a, b)| (a + 1) as u64 * *b as u64).sum()
}

fn get_input() -> (VecDeque<i32>, VecDeque<i32>) {
    let lines: Vec<_> = util::get_input_lines().collect();
    let parts: Vec<_> = lines.as_slice().split(|s| s.is_empty()).collect();
    return (VecDeque::from_iter(parts[0].iter().skip(1).map(|s| s.parse().unwrap())),
            VecDeque::from_iter(parts[1].iter().skip(1).map(|s| s.parse().unwrap())))
}
