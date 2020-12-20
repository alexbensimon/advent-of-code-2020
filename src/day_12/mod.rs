use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 25);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 286);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u16 {
    let entries = lines_from_file("src/day_12/input.txt");
    find_manhattan_distance_part_2(&entries)
}

fn part_2_test() -> u16 {
    let entries = lines_from_file("src/day_12/input-test.txt");
    find_manhattan_distance_part_2(&entries)
}

fn part_1() -> u16 {
    let entries = lines_from_file("src/day_12/input.txt");
    find_manhattan_distance(&entries)
}

fn part_1_test() -> u16 {
    let entries = lines_from_file("src/day_12/input-test.txt");
    find_manhattan_distance(&entries)
}

fn find_manhattan_distance_part_2(entries: &Vec<String>) -> u16 {
    let instructions: Vec<Instruction> = entries
        .iter()
        .map(|entry| entry_to_instruction(entry))
        .collect();

    let mut waypoint = Position { north: 1, east: 10 };
    let mut ship = Position { north: 0, east: 0 };

    for instruction in instructions {
        match instruction.action {
            'N' => {
                waypoint.north += instruction.value as i32;
            }
            'S' => {
                waypoint.north -= instruction.value as i32;
            }
            'E' => {
                waypoint.east += instruction.value as i32;
            }
            'W' => {
                waypoint.east -= instruction.value as i32;
            }
            'L' | 'R' => {
                waypoint = rotate_waypoint(&waypoint, &instruction);
            }
            'F' => {
                ship.north += waypoint.north * instruction.value as i32;
                ship.east += waypoint.east * instruction.value as i32;
            }
            _ => {}
        }
    }

    (ship.north.abs() + ship.east.abs()) as u16
}

fn rotate_waypoint(waypoint: &Position, instruction: &Instruction) -> Position {
    let rotation_count = instruction.value / 90;
    let mut current_waypoint = waypoint.clone();
    let mut new_waypoint = Position { north: 0, east: 0 };

    for _ in 0..rotation_count {
        new_waypoint = match instruction.action {
            'L' => Position {
                north: current_waypoint.east,
                east: -current_waypoint.north,
            },
            'R' => Position {
                north: -current_waypoint.east,
                east: current_waypoint.north,
            },
            _ => Position { north: 0, east: 0 },
        };
        current_waypoint = new_waypoint.clone();
    }

    new_waypoint
}

#[derive(Clone, Debug)]
struct Position {
    north: i32,
    east: i32,
}

fn find_manhattan_distance(entries: &Vec<String>) -> u16 {
    let instructions: Vec<Instruction> = entries
        .iter()
        .map(|entry| entry_to_instruction(entry))
        .collect();

    let mut direction = 'E';
    let mut north = 0;
    let mut east = 0;

    for instruction in instructions {
        match instruction.action {
            'N' | 'S' | 'E' | 'W' => {
                move_ship(&instruction, &mut north, &mut east);
            }
            'L' | 'R' => direction = change_direction(&direction, &instruction),
            'F' => {
                move_ship(
                    &Instruction {
                        action: direction,
                        value: instruction.value,
                    },
                    &mut north,
                    &mut east,
                );
            }

            _ => {}
        }
    }

    (north.abs() + east.abs()) as u16
}

fn move_ship(instruction: &Instruction, north: &mut i32, east: &mut i32) {
    match instruction.action {
        'N' => {
            *north += instruction.value as i32;
        }
        'S' => {
            *north -= instruction.value as i32;
        }
        'E' => {
            *east += instruction.value as i32;
        }
        'W' => {
            *east -= instruction.value as i32;
        }
        _ => {}
    }
}

fn change_direction(current_direction: &char, instruction: &Instruction) -> char {
    let directions = ['N', 'E', 'S', 'W'];
    let current_direction_index = directions
        .iter()
        .position(|direction| direction == current_direction)
        .unwrap();
    let index_shift = match instruction.action {
        'L' => -(instruction.value as isize) / 90,
        'R' => instruction.value as isize / 90,
        _ => 0,
    };
    let new_index = (current_direction_index as isize + index_shift) % 4;
    if new_index < 0 {
        return directions[(directions.len() as isize + new_index) as usize];
    }
    directions[new_index as usize]
}

#[derive(Debug)]
struct Instruction {
    action: char,
    value: u16,
}

fn entry_to_instruction(entry: &str) -> Instruction {
    Instruction {
        action: entry.chars().nth(0).unwrap(),
        value: entry[1..].parse().unwrap(),
    }
}
