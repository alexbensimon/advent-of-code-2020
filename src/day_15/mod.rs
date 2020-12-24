use std::{collections::HashMap, time::Instant};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 436);
    assert_eq!(part_1_test_2(), 1836);
    // println!("part_1 {:?}", part_1());

    // assert_eq!(part_2_test(), 175594);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u32 {
    let mut numbers_to_turn: HashMap<u32, u32> = HashMap::new();
    numbers_to_turn.insert(2, 1);
    numbers_to_turn.insert(20, 2);
    numbers_to_turn.insert(0, 3);
    numbers_to_turn.insert(4, 4);
    numbers_to_turn.insert(1, 5);

    find_last_number_spoken(&numbers_to_turn, 17, 6, 30000000)
}

fn part_2_test() -> u32 {
    let mut numbers_to_turn: HashMap<u32, u32> = HashMap::new();
    numbers_to_turn.insert(0, 1);
    numbers_to_turn.insert(3, 2);

    find_last_number_spoken(&numbers_to_turn, 6, 3, 30000000)
}

fn part_1() -> u32 {
    let mut numbers_to_turn: HashMap<u32, u32> = HashMap::new();
    numbers_to_turn.insert(2, 1);
    numbers_to_turn.insert(20, 2);
    numbers_to_turn.insert(0, 3);
    numbers_to_turn.insert(4, 4);
    numbers_to_turn.insert(1, 5);

    find_last_number_spoken(&numbers_to_turn, 17, 6, 2020)
}

fn part_1_test_2() -> u32 {
    let mut numbers_to_turn: HashMap<u32, u32> = HashMap::new();
    numbers_to_turn.insert(3, 1);
    numbers_to_turn.insert(1, 2);

    find_last_number_spoken(&numbers_to_turn, 2, 3, 2020)
}

fn part_1_test() -> u32 {
    let mut numbers_to_turn: HashMap<u32, u32> = HashMap::new();
    numbers_to_turn.insert(0, 1);
    numbers_to_turn.insert(3, 2);

    find_last_number_spoken(&numbers_to_turn, 6, 3, 2020)
}

fn find_last_number_spoken(
    initial_numbers_to_turn: &HashMap<u32, u32>,
    initial_number_spoken: u32,
    starting_turn: u32,
    ending_turn: u32,
) -> u32 {
    let mut numbers_to_turn = initial_numbers_to_turn.clone();
    let mut number_spoken = initial_number_spoken;
    for i in starting_turn..ending_turn {
        match numbers_to_turn.get(&number_spoken) {
            Some(turn) => {
                let previous_spoken = turn.clone();
                *numbers_to_turn.get_mut(&number_spoken).unwrap() = i;
                number_spoken = i - previous_spoken;
            }
            None => {
                numbers_to_turn.insert(number_spoken, i);
                number_spoken = 0;
            }
        }
    }
    number_spoken
}
