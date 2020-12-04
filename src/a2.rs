use std::fs::read_to_string;
use std::ops::RangeInclusive;

pub fn a2_1() {
    exec(|(range, char, password)| {
        let actual = password.chars().filter(|c| c == char).count();
        range.contains(&actual)
    })
}

pub fn a2_2() {
    exec(|(range, char, password)| {
        let first = password.chars().nth(range.start() - 1).unwrap();
        let second = password.chars().nth(range.end() - 1).unwrap();
        (first == *char) != (second == *char)
    });
}

fn exec<T: FnMut(&(RangeInclusive<usize>, char, &str)) -> bool>(filter: T) {
    let valid = read_to_string("inputs/input-2")
        .expect("Failed to read map")
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let range = split.next().unwrap();
            let letter = split.next().unwrap().chars().next().unwrap();
            let password = split.next().unwrap();

            let mut range_split = range.split('-');
            let start = range_split.next().unwrap().parse::<usize>().unwrap();
            let end = range_split.next().unwrap().parse::<usize>().unwrap();

            (start..=end, letter, password)
        })
        .filter(filter)
        .count();

    println!("{}", valid)
}
