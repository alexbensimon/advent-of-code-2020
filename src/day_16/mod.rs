use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs, time::Instant};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 71);
    // println!("part_1 {:#?}", part_1());

    println!("part_2_test {:#?}", part_2_test());

    let duration = start.elapsed();
    println!("Finished after {:#?}", duration);
}

fn part_2_test() -> u16 {
    let contents = fs::read_to_string("src/day_16/input-test.txt").unwrap();
    find_departure_values(&contents)
}

fn part_1() -> u16 {
    let contents = fs::read_to_string("src/day_16/input.txt").unwrap();
    find_ticket_scanning_error_rate(&contents)
}

fn part_1_test() -> u16 {
    let contents = fs::read_to_string("src/day_16/input-test.txt").unwrap();
    find_ticket_scanning_error_rate(&contents)
}

fn find_departure_values(contents: &str) -> u16 {
    let fields_rules: Vec<FieldRules> = parse_fields_rules(contents);
    let nearby_tickets = parse_nearby_tickets(contents);
    let mut fields_to_positions: HashMap<&str, u8> = HashMap::new();

    for field_rules in fields_rules.iter() {
        let mut position: u8 = 0;
        let mut is_correct_position = true;
        println!("field_rules {:#?}", field_rules);
        loop {
            println!("position {:#?}", position);
            for nearby_ticket in nearby_tickets.iter() {
                println!("nearby_ticket {:#?}", nearby_ticket);
                let value = nearby_ticket[position as usize];
                if !value_follows_field_rules(value, field_rules) {
                    is_correct_position = false;
                    position += 1;
                    break;
                }
            }
            if is_correct_position {
                fields_to_positions.insert(&field_rules.field, position);
                break;
            }
        }
    }
    println!("fields_to_positions {:#?}", fields_to_positions);
    0
}

fn parse_nearby_tickets(contents: &str) -> Vec<Box<[u16]>> {
    contents.split("nearby tickets:\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .filter(|value| !value.is_empty())
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_fields_rules(contents: &str) -> Vec<FieldRules> {
    contents
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .map(|entry| entry_to_field_rules(entry))
        .collect()
}

fn find_ticket_scanning_error_rate(contents: &str) -> u16 {
    let fields_rules: Vec<FieldRules> = parse_fields_rules(contents);

    let nearby_tickets_values: Vec<u16> =
        contents.split("nearby tickets:\n").collect::<Vec<&str>>()[1]
            .split("\n")
            .flat_map(|ticket| ticket.split(","))
            .filter(|value| !value.is_empty())
            .map(|value| value.parse().unwrap())
            .collect();

    let mut ticket_scanning_error_rate: u16 = 0;
    for ticket_value in nearby_tickets_values {
        let mut is_correct_value = false;
        for field_rules in fields_rules.iter() {
            // If the value validates one rule, we know the value is correct so we can break immediately without addind the value
            // Else we continue and if we reach the end of the rules without validating any rule, we know the value is not correct
            if value_follows_field_rules(ticket_value, field_rules) {
                is_correct_value = true;
                break;
            }
        }
        if !is_correct_value {
            ticket_scanning_error_rate += ticket_value;
        }
    }

    ticket_scanning_error_rate
}

fn value_follows_field_rules(value: u16, field_rules: &FieldRules) -> bool {
    (value >= field_rules.min_one && value <= field_rules.max_one)
        || (value >= field_rules.min_two && value <= field_rules.max_two)
}

#[derive(Debug)]
struct FieldRules {
    field: String,
    min_one: u16,
    max_one: u16,
    min_two: u16,
    max_two: u16,
}

fn entry_to_field_rules(entry: &str) -> FieldRules {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<field>[\w|\s]+): (?P<min_one>\d+)-(?P<max_one>\d+) or (?P<min_two>\d+)-(?P<max_two>\d+)$").unwrap();
    }

    let caps = RE.captures(entry).unwrap();

    FieldRules {
        field: caps["field"].to_string(),
        min_one: caps["min_one"].parse().unwrap(),
        max_one: caps["max_one"].parse().unwrap(),
        min_two: caps["min_two"].parse().unwrap(),
        max_two: caps["max_two"].parse().unwrap(),
    }
}
