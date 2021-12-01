extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::process::exit;

mod day_one;

fn main() {
    let available_days = vec![
        run_day_one,
    ];

    let matches = App::new("Advent of Code 2021")
        .version("0.1")
        .author("jakub")
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
