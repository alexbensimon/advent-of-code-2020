use crate::utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 5);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 8);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> i16 {
    let entries = lines_from_file("src/day_08/input.txt");
    compute_acc_after_termination(entries)
}

fn part_2_test() -> i16 {
    let entries = lines_from_file("src/day_08/input-test.txt");
    compute_acc_after_termination(entries)
}

fn part_1() -> i16 {
    let entries = lines_from_file("src/day_08/input.txt");
    compute_acc_before_infinite(entries)
}

fn part_1_test() -> i16 {
    let entries = lines_from_file("src/day_08/input-test.txt");
    compute_acc_before_infinite(entries)
}

fn compute_acc_after_termination(entries: Vec<String>) -> i16 {
    let instructions: Vec<Instruction> = entries
        .iter()
        .map(|entry| entry_to_instruction(entry))
        .collect();

    let mut i: i16 = 0;
    let mut index_executed_list: Vec<i16> = vec![];
    let mut acc: i16 = 0;
    let mut change_operation_index: i16 = 0;

    loop {
        while !index_executed_list.contains(&i) {
            if i as usize == instructions.len() {
                return acc;
            }
            index_executed_list.push(i);
            let instruction = &instructions[i as usize];
            match instruction.op.as_str() {
                "acc" => {
                    acc += instruction.val;
                    i += 1;
                }
                "jmp" => {
                    if i == change_operation_index {
                        i += 1
                    } else {
                        i += instruction.val;
                    }
                }
                "nop" => {
                    if i == change_operation_index {
                        i += instruction.val;
                    } else {
                        i += 1
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        change_operation_index += 1;
        acc = 0;
        i = 0;
        index_executed_list = vec![];
    }
}

fn compute_acc_before_infinite(entries: Vec<String>) -> i16 {
    let instructions: Vec<Instruction> = entries
        .iter()
        .map(|entry| entry_to_instruction(entry))
        .collect();

    let mut i: i16 = 0;
    let mut index_executed_list: Vec<i16> = vec![];
    let mut acc: i16 = 0;
    while !index_executed_list.contains(&i) {
        index_executed_list.push(i);
        let instruction = &instructions[i as usize];
        match instruction.op.as_str() {
            "acc" => {
                acc += instruction.val;
                i += 1;
            }
            "jmp" => {
                i += instruction.val;
            }
            _ => {
                i += 1;
            }
        }
    }

    acc
}

#[derive(Debug)]
struct Instruction {
    op: String,
    val: i16,
}

fn entry_to_instruction(entry: &str) -> Instruction {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<op>acc|nop|jmp)\s(?P<val>.*)$").unwrap();
    }

    let caps = RE.captures(entry).unwrap();

    Instruction {
        op: caps["op"].to_string(),
        val: caps["val"].parse().unwrap(),
    }
}
