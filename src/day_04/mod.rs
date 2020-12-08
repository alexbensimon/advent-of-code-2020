use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs, time::Instant};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 2);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test(), 4);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u16 {
    let contents = fs::read_to_string("src/day_04/input.txt").unwrap();
    get_valid_password_count(contents, is_password_valid_part_2)
}

fn part_2_test() -> u16 {
    let contents = fs::read_to_string("src/day_04/input-test-2.txt").unwrap();
    get_valid_password_count(contents, is_password_valid_part_2)
}

fn part_1() -> u16 {
    let contents = fs::read_to_string("src/day_04/input.txt").unwrap();
    get_valid_password_count(contents, is_password_valid_part_1)
}

fn part_1_test() -> u16 {
    let contents = fs::read_to_string("src/day_04/input-test.txt").unwrap();
    get_valid_password_count(contents, is_password_valid_part_1)
}

fn get_valid_password_count(
    contents: String,
    is_password_valid: fn(password_map: HashMap<&str, &str>) -> bool,
) -> u16 {
    let passwords = contents.split("\n\n").collect::<Vec<&str>>();

    let mut valid_password_count: u16 = 0;
    for password in passwords {
        let password_map = get_map_from_password(password);
        if is_password_valid(password_map) {
            valid_password_count += 1;
        }
    }
    valid_password_count
}

fn get_map_from_password(password: &str) -> HashMap<&str, &str> {
    let mut password_map: HashMap<&str, &str> = HashMap::new();

    let items: Vec<&str> = password
        .split("\n")
        .flat_map(|item| item.split(" "))
        .filter(|item| !item.is_empty())
        .collect();

    for item in items {
        let members: Vec<&str> = item.split(":").collect();
        password_map.insert(members[0], members[1]);
    }

    password_map
}

fn is_password_valid_part_1(password_map: HashMap<&str, &str>) -> bool {
    let keys_needed = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for key_needed in keys_needed.iter() {
        if !password_map.contains_key(key_needed) {
            return false;
        }
    }
    true
}

struct IntervalValue {
    key: String,
    min: u16,
    max: u16,
}

fn is_password_valid_part_2(password_map: HashMap<&str, &str>) -> bool {
    let interval_values = [
        IntervalValue {
            key: "byr".to_string(),
            min: 1920,
            max: 2002,
        },
        IntervalValue {
            key: "iyr".to_string(),
            min: 2010,
            max: 2020,
        },
        IntervalValue {
            key: "eyr".to_string(),
            min: 2020,
            max: 2030,
        },
    ];

    for interval_value in interval_values.iter() {
        match password_map.get(interval_value.key.as_str()) {
            None => return false,
            Some(value) => match value.parse::<u16>() {
                Err(_) => return false,
                Ok(val) => {
                    if val < interval_value.min || val > interval_value.max {
                        return false;
                    }
                }
            },
        }
    }

    match password_map.get("hgt") {
        None => return false,
        Some(hgt) => {
            lazy_static! {
                static ref RE_HGT: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
            }
            match RE_HGT.captures(hgt) {
                None => return false,
                Some(caps) => match &caps["unit"] {
                    "cm" => {
                        if caps["num"].parse::<u16>().unwrap() < 150
                            || caps["num"].parse::<u16>().unwrap() > 193
                        {
                            return false;
                        }
                    }
                    "in" => {
                        if caps["num"].parse::<u16>().unwrap() < 59
                            || caps["num"].parse::<u16>().unwrap() > 76
                        {
                            return false;
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    match password_map.get("hcl") {
        None => return false,
        Some(hcl) => {
            lazy_static! {
                static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            if !RE_HCL.is_match(hcl) {
                return false;
            }
        }
    }

    match password_map.get("ecl") {
        None => return false,
        Some(ecl) => {
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
                return false;
            }
        }
    }

    match password_map.get("pid") {
        None => return false,
        Some(pid) => {
            lazy_static! {
                static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            if !RE_PID.is_match(pid) {
                return false;
            }
        }
    }

    true
}
