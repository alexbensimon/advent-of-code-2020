use crate::utils::lines_from_file;
use std::{time::Instant, vec};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 820);

    // println!("part_1 {:?}", part_1());
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u16 {
    let entries = lines_from_file("src/day_05/input.txt");

    let mut seats_occupied = vec![vec![false; 8]; 127];
    let mut seat_id_list: Vec<u16> = vec![];

    for entry in entries {
        let row = compute_seat_row(entry[..7].to_string()) as usize;
        let column = compute_seat_column(entry[7..].to_string()) as usize;
        seats_occupied[row][column] = true;
        seat_id_list.push((row as u16) * 8 + (column as u16));
    }

    for row in 0..127 {
        for column in 0..8 {
            if !seats_occupied[row][column] {
                let seat_id = (row as u16) * 8 + (column as u16);
                if seat_id != 0
                    && seat_id_list.contains(&(seat_id - 1))
                    && seat_id_list.contains(&(seat_id + 1))
                {
                    return seat_id;
                }
            }
        }
    }

    1
}

fn part_1() -> u16 {
    let entries = lines_from_file("src/day_05/input.txt");
    find_highest_seat_id(entries)
}

fn part_1_test() -> u16 {
    let entries = lines_from_file("src/day_05/input-test.txt");
    find_highest_seat_id(entries)
}

fn find_highest_seat_id(entries: Vec<String>) -> u16 {
    let mut highest: u16 = 0;

    for entry in entries {
        let current = compute_seat_id(entry);
        if current > highest {
            highest = current
        }
    }

    highest
}

fn compute_seat_id(seat_code: String) -> u16 {
    let row = compute_seat_row(seat_code[..7].to_string());
    let column = compute_seat_column(seat_code[7..].to_string());
    (row as u16) * 8 + (column as u16)
}

fn compute_seat_row(row_code: String) -> u8 {
    let mut rows: Vec<u8> = (0..=127).collect();

    for i in 0..7 {
        match row_code.chars().nth(i).unwrap() {
            'F' => rows = rows[..(rows.len() / 2)].to_vec(),
            'B' => rows = rows[(rows.len() / 2)..].to_vec(),
            _ => {}
        }
    }

    *rows.first().unwrap()
}

fn compute_seat_column(column_code: String) -> u8 {
    let mut columns: Vec<u8> = (0..=7).collect();

    for i in 0..3 {
        match column_code.chars().nth(i).unwrap() {
            'L' => columns = columns[..(columns.len() / 2)].to_vec(),
            'R' => columns = columns[(columns.len() / 2)..].to_vec(),
            _ => {}
        }
    }

    *columns.first().unwrap()
}
