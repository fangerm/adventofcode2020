use std::fs::read_to_string;
use std::collections::HashSet;

pub fn a5_1() {
    println!("{}", a5().into_iter().max().unwrap());
}

const MAX_SEAT: u32 = (0b1111111 * 8) + 0b111;

pub fn a5_2() {
    let seats = a5().into_iter().collect::<HashSet<_>>();
    for num in 1..MAX_SEAT {
        if !seats.contains(&num) && seats.contains(&(num - 1)) && seats.contains(&(num + 1)) {
            println!("Missing seat number {} has seats around it!", num)
        }
    }
}

fn a5() -> Vec<u32> {
    // TODO: This allocation is subobtimal but lifetimes suck
    read_to_string("inputs/input-5")
        .expect("Failed to read seats")
        .lines()
        .map(|seat| {
            let row = &seat[0..7];
            let column = &seat[7..10];
            let row_bin = row.replace('B', "1").replace('F', "0");
            let column_bin = column.replace('R', "1").replace('L', "0");
            let row_num = u32::from_str_radix(&row_bin, 2).unwrap();
            let column_num = u32::from_str_radix(&column_bin, 2).unwrap();
            (row_num * 8) + column_num
        })
        .collect::<Vec<_>>()
}