use std::{collections::HashSet, fs::read_to_string};

pub fn a4_1() {
    a4(false)
}

pub fn a4_2() {
    a4(true)
}

fn a4(validation: bool) {
    let mut keyset = HashSet::<&str>::new();
    let valid_count = read_to_string("inputs/input-4")
        .expect("Failed to read map")
        .split("\n\n")
        .map(|passport| {
            keyset.clear();
            passport.split(|c| c == ' ' || c == '\n').for_each(|entry| {
                let mut split = entry.split(':');
                let key = split.next().unwrap_or("");
                let value = split.next().unwrap_or("");
                if !validation || validate(key, value) {
                    keyset.insert(key);
                }
            });
            keyset.len() == 8 || (keyset.len() == 7 && !keyset.contains("cid"))
        })
        .filter(|valid| *valid)
        .count();
    println!("{}", valid_count);
}

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),

        "hgt" if value.ends_with('m') && value.len() == 5 => {
            // CM
            (150..=193).contains(&value[0..3].parse().unwrap_or(0))
        }
        "hgt" if value.ends_with('n') && value.len() == 4 => {
            // INCH
            (59..=76).contains(&value[0..2].parse().unwrap_or(0))
        }

        "hcl" => value.starts_with('#') && value.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|color| value == *color),

        "pid" => value.chars().all(|c| c.is_ascii_digit()) && value.len() == 9,
        "cid" => true,

        _ => false,
    }
}
