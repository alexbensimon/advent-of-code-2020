use std::{collections::HashSet, fs, time::Instant};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 11);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 6);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u16 {
    let contents = fs::read_to_string("src/day_06/input.txt").unwrap();
    compute_every_yes_count(contents)
}

fn part_2_test() -> u16 {
    let contents = fs::read_to_string("src/day_06/input-test.txt").unwrap();
    compute_every_yes_count(contents)
}

fn part_1() -> u16 {
    let contents = fs::read_to_string("src/day_06/input.txt").unwrap();
    compute_any_yes_count(contents)
}

fn part_1_test() -> u16 {
    let contents = fs::read_to_string("src/day_06/input-test.txt").unwrap();
    compute_any_yes_count(contents)
}

fn compute_any_yes_count(contents: String) -> u16 {
    let groups = contents
        .split("\n\n")
        .map(|group| group.replace("\n", ""))
        .collect::<Vec<String>>();

    let mut yes_count: u16 = 0;

    for group in groups {
        let set = group.chars().collect::<HashSet<char>>();
        yes_count += set.len() as u16;
    }

    yes_count
}

fn compute_every_yes_count(contents: String) -> u16 {
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    let mut yes_count: u16 = 0;

    for group in groups {
        let people_answers = group
            .split("\n")
            .filter(|answers| !answers.is_empty())
            .collect::<Vec<&str>>();

        let person_ref_answers = people_answers[0];
        for answer in person_ref_answers.chars() {
            if everyone_answered_yes(&people_answers, answer) {
                yes_count += 1;
            }
        }
    }

    yes_count
}

fn everyone_answered_yes(people_answers: &Vec<&str>, answer: char) -> bool {
    for person_answers in people_answers {
        if !person_answers.contains(answer) {
            return false;
        }
    }
    true
}
