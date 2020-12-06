use crate::utils::lines_from_file;
use lazy_static::lazy_static;
use regex::{Error, Regex};
use std::{str::FromStr, time::Instant};

pub fn main() {
    let start = Instant::now();

    let entries = lines_from_file("src/day_02/input.txt");

    println!("valid_count {:?}", part_2(entries));

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

fn part_2(entries: Vec<String>) -> i32 {
    let mut valid_count = 0;

    for entry in entries {
        let line = Line::from_str(entry.as_str()).unwrap();
        let char_a = line.password.chars().nth((line.min - 1) as usize).unwrap();
        let char_b = line.password.chars().nth((line.max - 1) as usize).unwrap();

        if (char_a.eq(&line.letter) && !char_b.eq(&line.letter))
            || !char_a.eq(&line.letter) && char_b.eq(&line.letter)
        {
            valid_count += 1;
        }
    }

    valid_count
}

fn part_1(entries: Vec<String>) -> i32 {
    let mut valid_count = 0;

    for entry in entries {
        let line = Line::from_str(entry.as_str()).unwrap();
        let re = Regex::new(line.letter.to_string().as_str()).unwrap();
        let caps = re.captures_iter(line.password.as_str());
        let match_count = caps.into_iter().collect::<Vec<_>>().len() as u8;

        if match_count >= line.min && match_count <= line.max {
            valid_count += 1;
        }
    }

    valid_count
}

struct Line {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<min>\d{1,2})-(?P<max>\d{1,2})\s(?P<letter>\w):\s(?P<password>\w*)"
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        Ok(Line {
            min: caps["min"].parse::<u8>().unwrap(),
            max: caps["max"].parse::<u8>().unwrap(),
            letter: caps["letter"].chars().next().unwrap(),
            password: caps["password"].to_string(),
        })
    }
}
