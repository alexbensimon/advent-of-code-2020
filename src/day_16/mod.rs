use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, time::Instant};

pub fn main() {
    let start = Instant::now();

    let contents = fs::read_to_string("src/day_16/input.txt").unwrap();
    let fields_rules: Vec<FieldRules> = contents
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .map(|entry| entry_to_field_rules(entry))
        .collect();

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
            if (ticket_value >= field_rules.min_one && ticket_value <= field_rules.max_one)
                || (ticket_value >= field_rules.min_two && ticket_value <= field_rules.max_two)
            {
                is_correct_value = true;
                break;
            }
        }
        if !is_correct_value {
            ticket_scanning_error_rate += ticket_value;
        }
    }

    println!(
        "ticket_scanning_error_rate {:#?}",
        ticket_scanning_error_rate
    );

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
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
