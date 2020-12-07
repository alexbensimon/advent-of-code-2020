use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 7);

    // println!("part_1 {:?}", part_1());
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u32 {
    let entries = lines_from_file("src/day_03/input.txt");
    let total_tree_count = compute_tree_count(&entries, 1, 1) as u32
        * compute_tree_count(&entries, 3, 1) as u32
        * compute_tree_count(&entries, 5, 1) as u32
        * compute_tree_count(&entries, 7, 1) as u32
        * compute_tree_count(&entries, 1, 2) as u32;
    total_tree_count
}

fn part_1() -> u16 {
    let entries = lines_from_file("src/day_03/input.txt");
    compute_tree_count(&entries, 3, 1)
}

fn part_1_test() -> u16 {
    let entries = lines_from_file("src/day_03/input-test.txt");
    compute_tree_count(&entries, 3, 1)
}

fn compute_tree_count(entries: &Vec<String>, right_steps: u8, down_steps: u8) -> u16 {
    let input_count_needed = compute_input_count_needed(entries, right_steps, down_steps);
    let full_input = generate_full_input(entries, input_count_needed);

    let mut tree_count: u16 = 0;
    let mut index_x: u16 = 0;
    let mut index_y: u16 = 0;

    while (index_y as usize) < full_input.len() {
        let position = full_input[index_y as usize]
            .chars()
            .nth(index_x as usize)
            .unwrap();

        if position.eq(&'#') {
            tree_count += 1;
        }

        index_x += right_steps as u16;
        index_y += down_steps as u16;
    }

    tree_count
}

fn compute_input_count_needed(entries: &Vec<String>, right_steps: u8, down_steps: u8) -> u16 {
    let row_count = entries.len() as f64;
    let col_count = entries.first().unwrap().len() as f64;

    ((right_steps as f64 * row_count) / (down_steps as f64 * col_count)).ceil() as u16
}

fn generate_full_input(entries: &Vec<String>, input_count_needed: u16) -> Vec<String> {
    entries
        .iter()
        .map(|entry| multiply_string(entry.as_str(), input_count_needed))
        .collect::<Vec<String>>()
}

fn multiply_string(text: &str, times: u16) -> String {
    let mut final_text = String::new();
    for _ in 1..=times {
        final_text.push_str(text);
    }
    final_text
}
