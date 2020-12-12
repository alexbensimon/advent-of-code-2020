use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 127);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 62);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u64 {
    let entries = lines_from_file("src/day_09/input.txt");
    find_weakness(&entries, 1504371145)
}

fn part_2_test() -> u64 {
    let entries = lines_from_file("src/day_09/input-test.txt");
    find_weakness(&entries, 127)
}

fn part_1() -> u64 {
    let entries = lines_from_file("src/day_09/input.txt");
    find_first_not_sum(&entries, 25)
}

fn part_1_test() -> u64 {
    let entries = lines_from_file("src/day_09/input-test.txt");
    find_first_not_sum(&entries, 5)
}

fn find_weakness(entries: &Vec<String>, target: u64) -> u64 {
    let numbers: Vec<u64> = entries.iter().map(|line| line.parse().unwrap()).collect();

    let mut index_a: usize = 0;
    loop {
        let mut index_b = index_a + 1;
        loop {
            if index_b == numbers.len() {
                break;
            }
            let slice = &numbers[index_a..=index_b];
            let sum = slice.into_iter().fold(0, |a, b| a + b);
            if sum == target {
                return slice.into_iter().min().unwrap() + slice.into_iter().max().unwrap();
            }
            index_b += 1;
        }
        index_a += 1;
    }
}

fn find_first_not_sum(entries: &Vec<String>, preamble: u8) -> u64 {
    let numbers: Vec<u64> = entries.iter().map(|line| line.parse().unwrap()).collect();

    let mut i = preamble as usize;
    loop {
        let target = numbers[i];
        if !is_sum(target, &numbers[(i - (preamble as usize))..i]) {
            return target;
        }
        i += 1;
    }
}

fn is_sum(target: u64, choices: &[u64]) -> bool {
    for choice_a in choices {
        for choice_b in choices {
            if choice_a + choice_b == target && choice_a != choice_b {
                return true;
            }
        }
    }
    return false;
}
