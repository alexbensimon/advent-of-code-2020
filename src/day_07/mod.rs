use crate::utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
    vec,
};

pub fn main() {
    let start = Instant::now();

    assert_eq!(part_1_test(), 4);
    // println!("part_1 {:?}", part_1());

    assert_eq!(part_2_test_1(), 32);
    assert_eq!(part_2_test_2(), 126);
    println!("part_2 {:?}", part_2());

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2() -> u16 {
    let entries = lines_from_file("src/day_07/input.txt");
    count_bags(&entries)
}

fn part_2_test_2() -> u16 {
    let entries = lines_from_file("src/day_07/input-test-2.txt");
    count_bags(&entries)
}

fn part_2_test_1() -> u16 {
    let entries = lines_from_file("src/day_07/input-test.txt");
    count_bags(&entries)
}

fn part_1() -> u16 {
    let entries = lines_from_file("src/day_07/input.txt");
    count_bag_colors(&entries)
}

fn part_1_test() -> u16 {
    let entries = lines_from_file("src/day_07/input-test.txt");
    count_bag_colors(&entries)
}

fn count_bags(entries: &Vec<String>) -> u16 {
    let containers_map = build_containers_map(entries);

    let mut bag_count: u16 = 0;

    let starter_bag = Bags {
        count: 1,
        color: "shiny-gold".to_string(),
    };
    let mut keys_to_check: Vec<Bags> = vec![starter_bag];
    let mut i: usize = 0;

    while i < keys_to_check.len() {
        match containers_map.get(&keys_to_check[i].color) {
            Some(colors_contained) => {
                for color_contained in colors_contained {
                    let color_contained_bag_count = &keys_to_check[i].count * color_contained.count;
                    bag_count += color_contained_bag_count;
                    let next_bags = Bags {
                        count: color_contained_bag_count,
                        color: color_contained.color.clone(),
                    };
                    keys_to_check.push(next_bags);
                }
            }
            None => {}
        };
        i += 1;
    }

    bag_count
}

fn count_bag_colors(entries: &Vec<String>) -> u16 {
    let colors_contained_map = build_colors_contained_map(&entries);

    let mut bag_colors: HashSet<String> = HashSet::new();

    let mut keys_to_check = vec!["shiny-gold"];
    let mut i: usize = 0;

    while i < keys_to_check.len() {
        match colors_contained_map.get(keys_to_check[i]) {
            Some(containers) => {
                for container in containers {
                    bag_colors.insert(container.to_string());
                    keys_to_check.push(container);
                }
            }
            None => {}
        };
        i += 1;
    }

    bag_colors.len() as u16
}

fn build_containers_map(entries: &Vec<String>) -> HashMap<String, Vec<Bags>> {
    let mut containers_map: HashMap<String, Vec<Bags>> = HashMap::new();

    for entry in entries {
        let (container, colors_contained) = parse_rule_with_count(entry);
        containers_map.insert(container, colors_contained);
    }

    containers_map
}

fn build_colors_contained_map(entries: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut colors_contained_map: HashMap<String, HashSet<String>> = HashMap::new();
    for entry in entries {
        let (container, colors_contained) = parse_rule(entry);
        for color_contained in colors_contained {
            colors_contained_map
                .entry(color_contained)
                .or_insert(vec![container.clone()].into_iter().collect())
                .insert(container.clone());
        }
    }
    colors_contained_map
}
#[derive(Debug)]
struct Bags {
    count: u16,
    color: String,
}

fn parse_rule_with_count(rule: &str) -> (String, Vec<Bags>) {
    let parts: Vec<&str> = rule.split("bags contain").collect();

    let container = parts[0].trim().replace(" ", "-");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<count>\d)\s(?P<color>[^,]*)bag").unwrap();
    }
    let mut colors_contained: Vec<Bags> = vec![];
    for cap in RE.captures_iter(parts[1]) {
        colors_contained.push(Bags {
            count: cap["count"].parse().unwrap(),
            color: cap["color"].to_string().trim().replace(" ", "-"),
        })
    }

    (container, colors_contained)
}

fn parse_rule(rule: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = rule.split("bags contain").collect();

    let container = parts[0].trim().replace(" ", "-");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d\s([^,]*)bag").unwrap();
    }
    let mut colors_contained: Vec<String> = vec![];
    for cap in RE.captures_iter(parts[1]) {
        colors_contained.push(cap[1].to_string().trim().replace(" ", "-"));
    }

    (container, colors_contained)
}
