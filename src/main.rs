extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::process::exit;

const APP_NAME: &str = "Advent of Code 2021";
const VERSION: &str = "0.1";

mod day_one;
mod day_two;

fn main() {
    let available_days: Vec<fn()> = vec![
        run_day_one,
        run_day_two,
    ];

    let matches = App::new(APP_NAME)
        .version(VERSION)
        .arg(
            Arg::new("days")
                .short('d')
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
    let mut v: Vec<i32> = vec![];
    for line in reader.lines() {
        v.push(line.unwrap()
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e)).unwrap());
    }
    println!(
        "Day one part one answer is {}",
        day_one::get_num_increases(v.clone()),
    );
    println!(
        "Day one part two answer is {}",
        day_one::get_sliding_window_increases(v.clone()),
    );

}

fn run_day_two() {
    let day_two_file = match File::open("data/day_two.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day two file: {}", e),
    };
    let reader = BufReader::new(day_two_file);
    let mut v: Vec<day_two::Command> = vec![];
    for line in reader.lines() {
        v.push(convert_line_to_direction(line.unwrap()));
    }

    println!(
        "Day two part one answer is {}",
        day_two::get_cmd_product(v.clone()),
    );
    println!(
        "Day two part two answer is {}",
        day_two::get_aim_product(v.clone()),
    );
}

fn convert_line_to_direction(line: String) -> day_two::Command {
    let re = Regex::new(r"(?P<direction>\S+)\s+(?P<unit>\d+)").unwrap();
    let caps = re.captures(line.trim()).unwrap();

    let direction = match &caps["direction"] {
        day_two::FORWARD => day_two::Direction::Forward,
        day_two::DOWN => day_two::Direction::Down,
        day_two::UP => day_two::Direction::Up,
        s => panic!("unrecognized direction: {}", s),
    };

    let unit: i32 = caps["unit"].parse::<i32>().unwrap();

    return day_two::Command::new(direction, unit);
}