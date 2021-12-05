use std::fmt;

pub const FORWARD: &str = "forward";
pub const UP: &str = "up";
pub const DOWN: &str = "down";

#[derive(Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Direction::Forward => FORWARD,
            Direction::Down => DOWN,
            Direction::Up => UP,
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct Command {
    direction: Direction,
    unit: i32,
}

impl Command {
    pub fn new(direction: Direction, unit: i32) -> Self {
        Self { direction, unit }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.unit)
    }
}

pub fn get_cmd_product(cmds: Vec<Command>) -> i32 {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for cmd in cmds.iter() {
        let unit = cmd.unit;

        match cmd.direction {
            Direction::Forward => {
                horizontal += unit;
            }
            Direction::Down => {
                depth += unit;
            }
            Direction::Up => {
                depth -= unit;
            }
        }
    }

    return horizontal * depth;
}

pub fn get_aim_product(cmds: Vec<Command>) -> i32 {
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for cmd in cmds.iter() {
        let unit = cmd.unit;

        match cmd.direction {
            Direction::Forward => {
                horizontal += unit;
                depth += aim * unit;
            }
            Direction::Up => {
                aim -= unit;
            }
            Direction::Down => {
                aim += unit;
            }
        }
    }

    return depth * horizontal;
}

#[cfg(test)]
mod tests {
    use super::{get_aim_product, get_cmd_product, Command, Direction};

    #[test]
    fn test_cmd_product() {
        let cmds = vec![
            Command::new(Direction::Forward, 5),
            Command::new(Direction::Down, 5),
            Command::new(Direction::Forward, 8),
            Command::new(Direction::Up, 3),
            Command::new(Direction::Down, 8),
            Command::new(Direction::Forward, 2),
        ];

        assert_eq!(150, get_cmd_product(cmds));
    }

    #[test]
    fn test_aim_product() {
        let cmds = vec![
            Command::new(Direction::Forward, 5),
            Command::new(Direction::Down, 5),
            Command::new(Direction::Forward, 8),
            Command::new(Direction::Up, 3),
            Command::new(Direction::Down, 8),
            Command::new(Direction::Forward, 2),
        ];

        assert_eq!(900, get_aim_product(cmds));
    }
}
