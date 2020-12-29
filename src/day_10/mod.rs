use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    // assert_eq!(part_1_test_1(), 35);
    // assert_eq!(part_1_test_2(), 220);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test_1(), 8);
    assert_eq!(part_2_test_2(), 19208);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u64 {
    let entries = lines_from_file("src/day_10/input.txt");
    find_arrangement_count(&entries)
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

    let mut arrangement_count: u64 = 1;
    let mut i: usize = 0;
    loop {
        let mut j: usize = i + 1;
        let mut options: u32 = 0;
        loop {
            if j >= numbers.len() {
                break;
            }
            if j - i == 1 && numbers[j] - numbers[i] == 3 {
                break;
            }
            options += 1;
            if j + 1 >= numbers.len() || numbers[j + 1] - numbers[j] == 3 {
                break;
            }
            j += 1;
        }
        if options > 0 {
            let mut possibilities = 2u64.pow(options - 1);
            // When there are more than 3 options, the first and last numbers can't be connected because their difference must me > 3, so there is one less possibility
            if options > 3 {
                possibilities -= 1;
            }
            arrangement_count *= possibilities;
        }
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
