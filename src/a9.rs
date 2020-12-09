use std::fs::read_to_string;

const PREAMBLE: usize = 25;

pub fn a9_1() {
    let nums = load_input();
    println!("{}", nums[find_invalid_index(&nums)]);
}

pub fn a9_2() {
    let nums = load_input();
    let invalid = find_invalid_index(&nums);

    for start in 0..nums.len() {
        for end in start..nums.len() {
            let range = &nums[start..end];
            if range.iter().sum::<u64>() == nums[invalid] {
                let max = range.iter().max().unwrap();
                let min = range.iter().min().unwrap();
                println!("{}", max + min);
                return;
            }
        }
    }
}

fn load_input() -> Vec<u64> {
    read_to_string("inputs/input-9")
        .expect("Failed to read port data")
        .lines()
        .map(str::parse::<_>)
        .collect::<Result<Vec<u64>, _>>()
        .unwrap()
}

fn find_invalid_index(nums: &[u64]) -> usize {
    for i in PREAMBLE..nums.len() {
        let num = nums[i];
        let mut hit = false;
        for j in (i - PREAMBLE)..i {
            for k in (j + 1)..i {
                hit |= (nums[j] + nums[k]) == num;
            }
        }
        if !hit {
            return i;
        }
    }
    panic!("valid?")
}
