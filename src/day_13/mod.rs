use crate::utils::lines_from_file;
use std::time::Instant;

pub fn main() {
    let start = Instant::now();

    // assert_eq!(part_1_test(), 295);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 1068781);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u64 {
    let entries = lines_from_file("src/day_13/input.txt");
    compute_res_part_2(&entries, 100000000000000)
}

fn part_2_test() -> u64 {
    let entries = lines_from_file("src/day_13/input-test.txt");
    compute_res_part_2(&entries, 939)
}

fn part_1() -> u32 {
    let entries = lines_from_file("src/day_13/input.txt");
    compute_res(&entries)
}

fn part_1_test() -> u32 {
    let entries = lines_from_file("src/day_13/input-test.txt");
    compute_res(&entries)
}

fn compute_res_part_2(entries: &Vec<String>, starting_time: u64) -> u64 {
    let bus_id_list: Vec<&str> = entries[1].split(",").collect();

    let mut time: u64 = starting_time;
    'outer: loop {
        for (i, bus_id) in bus_id_list.iter().enumerate() {
            if bus_id != &"x" && (time + i as u64) % bus_id.parse::<u64>().unwrap() != 0 {
                time += 1;
                continue 'outer;
            }
        }
        break;
    }

    time
}

fn compute_res(entries: &Vec<String>) -> u32 {
    let earliest_time: u32 = entries[0].parse().unwrap();
    let bus_id_list = get_bus_id_list_from_entry(&entries[1]);

    let mut departure = earliest_time;
    let mut earliest_bus: u16 = 0;
    while earliest_bus == 0 {
        departure += 1;
        for bus_id in &bus_id_list {
            if departure % *bus_id as u32 == 0 {
                earliest_bus = *bus_id;
                break;
            }
        }
    }

    (departure - earliest_time) * earliest_bus as u32
}

fn get_bus_id_list_from_entry(entry: &str) -> Vec<u16> {
    entry
        .split(",")
        .filter(|id| id != &"x")
        .map(|id| id.parse().unwrap())
        .collect()
}
