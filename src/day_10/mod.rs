use crate::utils::lines_from_file;
use std::{cmp::max, time::Instant};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test_1(), 35);
    assert_eq!(part_1_test_2(), 220);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test_1(), 8);
    assert_eq!(part_2_test_2(), 19208);
    // TODO: PART 2 NOT FINISHED

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2_test_2() -> u64 {
    let entries = lines_from_file("src/day_10/input-test-2.txt");
    find_arrangement_count(&entries)
}

fn part_2_test_1() -> u64 {
    let entries = lines_from_file("src/day_10/input-test.txt");
    find_arrangement_count(&entries)
}

fn part_1() -> u16 {
    let entries = lines_from_file("src/day_10/input.txt");
    find_jolt_diff_product(&entries)
}

fn part_1_test_2() -> u16 {
    let entries = lines_from_file("src/day_10/input-test-2.txt");
    find_jolt_diff_product(&entries)
}

fn part_1_test_1() -> u16 {
    let entries = lines_from_file("src/day_10/input-test.txt");
    find_jolt_diff_product(&entries)
}

fn find_arrangement_count(entries: &Vec<String>) -> u64 {
    let mut numbers: Vec<u16> = entries.iter().map(|entry| entry.parse().unwrap()).collect();
    numbers.push(0); // For the charging outlet
    numbers.sort();

    println!("numbers {:#?}", numbers);

    let mut arrangement_count: u64 = 1;
    let mut i: usize = 0;
    loop {
        println!("i {:?}", numbers[i]);
        let mut j: usize = i + 1;
        let mut possibilities: u64 = 0;
        loop {
            if j < numbers.len() {
                println!("j {:?}", numbers[j]);
            }
            if j >= numbers.len() {
                break;
            }
            if numbers[j] - numbers[i + 1] == 2 {
                possibilities += 1;
            }
            if numbers[j] - numbers[i] == 3 {
                possibilities += 1;
                break;
            }
            if numbers[j] - numbers[i] > 3 {
                break;
            }
            possibilities += 1;

            j += 1;
        }
        println!("possibilities {:?}", possibilities);

        arrangement_count *= max(1, possibilities);
        println!("arrangement_count {:?}", arrangement_count);
        i = j;
        if i >= numbers.len() {
            break;
        }
    }

    arrangement_count
}

fn find_jolt_diff_product(entries: &Vec<String>) -> u16 {
    let mut numbers: Vec<u16> = entries.iter().map(|entry| entry.parse().unwrap()).collect();
    numbers.push(0); // For the charging outlet
    numbers.sort();

    let mut one_diff_count: u16 = 0;
    let mut three_diff_count: u16 = 1; // Starts at 1 for the built-in adapter

    let mut i: usize = 1;
    while i < numbers.len() {
        if numbers[i] - numbers[i - 1] == 3 {
            three_diff_count += 1;
        } else {
            one_diff_count += 1;
        }
        i += 1;
    }

    one_diff_count * three_diff_count
}

fn find_diff_counts(entries: &Vec<String>) {
    let mut numbers: Vec<u16> = entries.iter().map(|entry| entry.parse().unwrap()).collect();
    numbers.push(0); // For the charging outlet
    numbers.sort();

    println!("numbers {:#?}", numbers);

    let mut one_diff_count: u16 = 0;
    let mut two_diff_count: u16 = 0;
    let mut three_diff_count: u16 = 1; // Starts at 1 for the built-in adapter

    let mut i: usize = 1;
    while i < numbers.len() {
        if numbers[i] - numbers[i - 1] == 3 {
            three_diff_count += 1;
        } else if numbers[i] - numbers[i - 1] == 2 {
            two_diff_count += 1;
        } else {
            one_diff_count += 1;
        }
        i += 1;
    }

    println!("one_diff_count {:#?}", one_diff_count);
    println!("two_diff_count {:#?}", two_diff_count);
    println!("three_diff_count {:#?}", three_diff_count);
}
