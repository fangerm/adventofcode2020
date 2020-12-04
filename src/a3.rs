use std::fs::read_to_string;

pub fn a3_1() {
    a3(&[3], &[1]);
}

pub fn a3_2() {
    let res = a3(&[1, 3, 5, 7, 1], &[1, 1, 1, 1, 2]);
    println!("{}", res);
}

pub fn a3(rights: &[usize], downs: &[usize]) -> usize {
    let trees = read_to_string("inputs/input-3")
        .expect("Failed to read map")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<_>>>();

    rights
        .iter()
        .zip(downs.iter())
        .fold(1usize, |res, (right, down)| {
            let mut count = 0;
            let mut index = 0;
            for row in trees.iter().step_by(*down) {
                count += row[index % row.len()] as usize;
                index += right;
            }

            println!("right: {}  down: {}  trees: {}", right, down, count);
            res * count
        })
}
