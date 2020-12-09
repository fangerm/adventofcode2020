use std::fs::read_to_string;
use std::collections::{HashSet, HashMap};

pub fn a6_1() {
    let mut answered = HashSet::<char>::with_capacity(26);
    let count: usize = read_to_string("inputs/input-6")
        .expect("Failed to read answers")
        .split("\n\n")
        .map(|group| {
            for char in group.lines().flat_map(|line| line.chars()) {
                answered.insert(char);
            }
            let len = answered.len();
            answered.clear();
            len
        })
        .sum();
    println!("{}", count);
}

pub fn a6_2() {
    let mut counts = HashMap::<char, usize>::with_capacity(26);
    let count: usize = read_to_string("inputs/input-6")
        .expect("Failed to read answers")
        .split("\n\n")
        .map(|group| {
            for char in group.lines().flat_map(|line| line.chars()) {
                let count = *counts.get(&char).unwrap_or(&0);
                counts.insert(char, count + 1);
            }
            let group_size = group.lines().count();
            let count = counts.iter().filter(|(_, count)| **count == group_size).count();
            counts.clear();
            count
        })
        .sum();
    println!("{}", count);
}