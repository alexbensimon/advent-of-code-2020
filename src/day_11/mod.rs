use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 37);
    // println!("part_1 {:?}", part_1());

    assert_eq!(mini_test().len(), 0);
    assert_eq!(part_2_test(), 26);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u32 {
    let entries = lines_from_file("src/day_11/input.txt");
    count_final_occupied_seats(&entries, get_values_around_position_part_2, 5)
}

fn mini_test() -> Vec<char> {
    let entries = lines_from_file("src/day_11/mini-test.txt");
    let seats: Vec<Vec<char>> = entries
        .iter()
        .map(|entry| entry.chars().collect())
        .collect();
    get_values_around_position_part_2(
        &seats,
        Position {
            row_index: 3,
            column_index: 3,
        },
    )
}

fn part_2_test() -> u32 {
    let entries = lines_from_file("src/day_11/input-test.txt");
    count_final_occupied_seats(&entries, get_values_around_position_part_2, 5)
}

fn part_1() -> u32 {
    let entries = lines_from_file("src/day_11/input.txt");
    count_final_occupied_seats(&entries, get_values_around_position, 4)
}

fn part_1_test() -> u32 {
    let entries = lines_from_file("src/day_11/input-test.txt");
    count_final_occupied_seats(&entries, get_values_around_position, 4)
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    row_index: usize,
    column_index: usize,
}

fn count_final_occupied_seats(
    entries: &Vec<String>,
    get_values: fn(seats: &Vec<Vec<char>>, position: Position) -> Vec<char>,
    occupied_threshold: u8,
) -> u32 {
    let seats: Vec<Vec<char>> = entries
        .iter()
        .map(|entry| entry.chars().collect())
        .collect();

    let row_count = entries.len();
    let column_count = entries.first().unwrap().len();

    let mut current_round = seats.clone();
    let mut next_round = vec![vec!['.'; column_count]; row_count];
    let last_round = loop {
        let mut has_changed = false;

        for row_index in 0..row_count {
            for column_index in 0..column_count {
                let values = get_values(
                    &current_round,
                    Position {
                        row_index,
                        column_index,
                    },
                );
                match current_round[row_index][column_index] {
                    'L' => {
                        if !values.contains(&'#') {
                            next_round[row_index][column_index] = '#';
                            has_changed = true;
                        }
                    }
                    '#' => {
                        if values.iter().filter(|val| **val == '#').count()
                            >= occupied_threshold as usize
                        {
                            next_round[row_index][column_index] = 'L';
                            has_changed = true;
                        }
                    }
                    _ => {
                        next_round[row_index][column_index] =
                            current_round[row_index][column_index];
                    }
                }
            }
        }

        if !has_changed {
            break next_round;
        }
        current_round = next_round.clone();
    };

    last_round
        .iter()
        .map(|row| {
            row.iter().fold(0u32, |count, seat| {
                if seat == &'#' {
                    return count + 1;
                }
                count
            })
        })
        .sum()
}

fn get_values_around_position_part_2(seats: &Vec<Vec<char>>, position: Position) -> Vec<char> {
    let mut values: Vec<char> = vec![];

    for i in -1i32..=1 {
        for j in -1i32..=1 {
            let mut row_shift = 0;
            let mut column_shift = 0;
            loop {
                row_shift += i;
                column_shift += j;
                let seat_row = position.row_index as i32 + row_shift;
                let seat_column = position.column_index as i32 + column_shift;
                if seat_row < 0 || seat_column < 0 {
                    break;
                }
                let seat = Position {
                    row_index: seat_row as usize,
                    column_index: seat_column as usize,
                };
                if !position_exists(seats, &seat) || seat.eq(&position) {
                    break;
                }
                let value = seats[seat.row_index][seat.column_index];
                if value != '.' {
                    values.push(value);
                    break;
                }
            }
        }
    }

    values
}

fn get_values_around_position(seats: &Vec<Vec<char>>, position: Position) -> Vec<char> {
    let seats_to_check = get_seats_to_check_around_position(seats, position);
    seats_to_check
        .iter()
        .map(|seat| seats[seat.row_index][seat.column_index])
        .collect()
}

fn get_seats_to_check_around_position(seats: &Vec<Vec<char>>, position: Position) -> Vec<Position> {
    let mut seats_to_check: Vec<Position> = vec![];
    for i in -1i8..=1 {
        for j in -1i8..=1 {
            let seat_row = position.row_index as i8 + i;
            let seat_column = position.column_index as i8 + j;
            if seat_row >= 0 && seat_column >= 0 {
                let seat = Position {
                    row_index: seat_row as usize,
                    column_index: seat_column as usize,
                };
                if position_exists(seats, &seat) && !seat.eq(&position) {
                    seats_to_check.push(seat);
                }
            }
        }
    }
    seats_to_check
}

fn position_exists(seats: &Vec<Vec<char>>, position: &Position) -> bool {
    match seats.get(position.row_index) {
        Some(row) => match row.get(position.column_index) {
            Some(_) => return true,
            None => return false,
        },
        None => return false,
    }
}
