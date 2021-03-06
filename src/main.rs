extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::process::exit;

const APP_NAME: &str = "Advent of Code 2021";
const VERSION: &str = "0.1";

mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_three;
mod day_two;

fn main() {
    let available_days: Vec<fn()> = vec![
        run_day_one,
        run_day_two,
        run_day_three,
        run_day_four,
        run_day_five,
        run_day_six,
        run_day_seven,
        run_day_eight,
        run_day_nine,
        run_day_ten,
    ];

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
        v.push(
            line.unwrap()
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                .unwrap(),
        );
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

    day_two::Command::new(direction, unit)
}

fn run_day_three() {
    let day_three_file = match File::open("data/day_three.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day three file: {}", e),
    };
    let reader = BufReader::new(day_three_file);
    let mut v: Vec<String> = vec![];
    for line in reader.lines() {
        v.push(line.unwrap().trim().to_string());
    }

    println!(
        "Day three part one answer is {}",
        day_three::get_gamma_epsilon_product(v.clone()),
    );
    println!(
        "Day three part one answer is {}",
        day_three::get_oxygen_co2_product(v.clone()),
    );
}

fn run_day_four() {
    let day_four_file = match File::open("data/day_four.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day four file: {}", e),
    };
    let reader = BufReader::new(day_four_file);
    let mut lines = reader.lines();
    let called_nums: Vec<i32> = match lines.next() {
        Some(s) => s
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect(),
        None => panic!("cannot read first line of day three file"),
    };
    lines.next();

    let mut boards: Vec<day_four::Board> = Vec::new();
    let mut nums: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let cur_line = line.unwrap();
        if cur_line.trim().is_empty() {
            boards.push(day_four::Board::new(nums.clone()));
            nums = Vec::new();
        } else {
            nums.push(
                cur_line
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }
    boards.push(day_four::Board::new(nums));

    println!(
        "Day four part one answer is {}",
        day_four::find_first_winning_board(boards.clone(), called_nums.clone()),
    );
    println!(
        "Day four part two answer is {}",
        day_four::find_last_winning_board(boards, called_nums),
    );
}

fn run_day_five() {
    let day_five_file = match File::open("data/day_five.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day five file: {}", e),
    };
    let reader = BufReader::new(day_five_file);
    let mut points: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in reader.lines() {
        let mut parsed_pts: Vec<(i32, i32)> = Vec::new();
        for split_pt in line.unwrap().split(" -> ") {
            let pt = split_pt
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            parsed_pts.push((pt[0], pt[1]));
        }
        points.push((parsed_pts[0], parsed_pts[1]));
    }

    println!(
        "Day five part one answer is {}",
        day_five::get_straight_overlap(points.clone()),
    );
    println!(
        "Day five part two answer is {}",
        day_five::get_all_overlap(points),
    );
}

fn run_day_six() {
    let day_six_file = match File::open("data/day_six.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day six file: {}", e),
    };

    let reader = BufReader::new(day_six_file);
    let nums: Vec<i64> = match reader.lines().next() {
        Some(s) => s
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
        None => panic!("cannot read first line of day six file"),
    };

    println!(
        "Day six part one answer is {}",
        day_six::calculate_lanternfish(nums.clone(), 80),
    );
    println!(
        "Day six part two answer is {}",
        day_six::calculate_lanternfish(nums, 256),
    );
}

fn run_day_seven() {
    let day_seven_file = match File::open("data/day_seven.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day seven file: {}", e),
    };

    let reader = BufReader::new(day_seven_file);
    let nums: Vec<i32> = match reader.lines().next() {
        Some(s) => s
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect(),
        None => panic!("cannot read first line of day seven file"),
    };

    println!(
        "Day seven part one answer is {}",
        day_seven::constant_gas(nums.clone()),
    );
    println!(
        "Day seven part two answer is {}",
        day_seven::variable_gas(nums),
    );
}

fn run_day_eight() {
    let day_eight_file = match File::open("data/day_eight.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day eight file: {}", e),
    };

    let reader = BufReader::new(day_eight_file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .collect();
    println!(
        "Day eight part one answer is {}",
        day_eight::get_unique_digits(lines),
    );
}

fn run_day_nine() {
    let day_nine_file = match File::open("data/day_nine.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day nine file: {}", e),
    };

    let reader = BufReader::new(day_nine_file);
    let pts: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();
    println!(
        "Day nine part one answer is {}",
        day_nine::sum_low_points(pts.clone()),
    );
    println!("Day nine part two answer is {}", day_nine::find_basins(pts));
}

fn run_day_ten() {
    let day_ten_file = match File::open("data/day_ten.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day ten file: {}", e),
    };

    let reader = BufReader::new(day_ten_file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mismatches: usize = lines
        .iter()
        .map(|l| day_ten::check_balanced(&l.clone()[..]))
        .sum();
    println!("Day ten part one answer is {}", mismatches,);
    println!(
        "Day ten part two answer is {}",
        day_ten::get_middle_completion_score(lines.clone()),
    );
}
