extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::exit;

const APP_NAME: &str = "Advent of Code 2020";
const VERSION: &str = "0.1";

mod day_one;
mod day_two;

fn main() {
    let available_days: Vec<fn()> = vec![run_day_one, run_day_two];

    let matches = App::new(APP_NAME)
        .version(VERSION)
        .arg(
            Arg::new("days")
                .short('d')
                .long("day")
                .value_name("DAYS")
                .about("Enter a comma-separated list of days")
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .get_matches();

    for day in matches.values_of_t::<usize>("days").unwrap().iter() {
        let idx = day - 1;
        if idx > available_days.len() {
            println!("day {} not available", day);
            exit(1);
        }
        available_days[idx]();
    }
}

fn run_day_one() {
    let day_one_file = match File::open("data/day_one.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day one file: {}", e),
    };
    let reader = BufReader::new(day_one_file);
    let v: Vec<i32> = reader
        .lines()
        .into_iter()
        .map(|l| l.ok().and_then(|s| s.parse().ok()).unwrap_or(0))
        .collect();
    println!(
        "Day one part one answer is {}",
        day_one::two_sum(v.clone(), 2020).unwrap(),
    );
    println!(
        "Day one part two answer is {}",
        day_one::three_sum(v, 2020).unwrap()
    );
}

fn run_day_two() {
    let day_two_file = match File::open("data/day_two.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day two file: {}", e),
    };
    let reader = BufReader::new(day_two_file);
    let policies: Vec<day_two::PasswordPolicy> = reader
        .lines()
        .map(|l| convert_line_to_policy(l.ok().unwrap()))
        .collect();
    println!(
        "Day two part one answer is {}",
        day_two::count_valid_passwords(policies.clone()),
    );
    println!(
        "Day two part two answer is {}",
        day_two::count_valid_position_passwords(policies),
    );
}

fn convert_line_to_policy(line: String) -> day_two::PasswordPolicy {
    let re = Regex::new(r"(?P<min>\d+)\-(?P<max>\d+) (?P<char>\S): (?P<password>\S+)").unwrap();
    let caps = re.captures(line.trim()).unwrap();

    let min: usize = caps["min"].parse().unwrap();
    let max: usize = caps["max"].parse().unwrap();
    let c: char = caps["char"].chars().next().unwrap();
    let password: String = caps["password"].to_owned();

    day_two::PasswordPolicy::new(c, min, max, password)
}
