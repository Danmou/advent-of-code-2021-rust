use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
enum Direction {
    up,
    down,
    forward,
}

fn read_input() -> Vec<(Direction, i64)> {
    let file = File::open("inputs/2.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut res = Vec::new();

    for line in br.lines() {
        let asdf = line.unwrap();
        let parts: Vec<&str> = asdf.split(" ").collect();
        match parts[..] {
            [a, b] => res.push((Direction::from_str(a).unwrap(), b.parse().unwrap())),
            _ => panic!(),
        }
    }
    res
}

fn update_position(commands: Vec<(Direction, i64)>) -> (i64, i64) {
    let mut pos = 0;
    let mut depth = 0;
    for command in commands.iter() {
        match command {
            (Direction::up, d) => depth -= d,
            (Direction::down, d) => depth += d,
            (Direction::forward, d) => pos += d,
        }
    }
    (pos, depth)
}

fn main() {
    let input = read_input();
    let (pos, depth) = update_position(input);
    // let input_sum = window_sum(input);
    // let count = count_increases(input_sum);
    println!("{}", pos * depth);
}
