use std::fs::File;
use std::io::{ prelude::*, BufReader, Error, ErrorKind };

mod day_one;

fn main() {
    run_day_one();
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
