use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    let entries: Vec<i32> = lines_from_file("src/day_01/input.txt")
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    // println!("{:?}", part_1(entries));

    println!("{:?}", part_2(entries));

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_1(entries: Vec<i32>) -> i32 {
    for entry_a in entries.iter() {
        for entry_b in entries.iter() {
            if entry_a + entry_b == 2020 {
                return entry_a * entry_b;
            }
        }
    }
    return 0;
}

fn part_2(entries: Vec<i32>) -> i32 {
    for entry_a in entries.iter() {
        for entry_b in entries.iter() {
            for entry_c in entries.iter() {
                if entry_a + entry_b + entry_c == 2020 {
                    return entry_a * entry_b * entry_c;
                }
            }
        }
    }
    return 0;
}
