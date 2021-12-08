use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
#[allow(non_camel_case_types)]
enum Segment {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
}

type DisplayState = HashSet<Segment>;

fn parse_set(s: &str) -> DisplayState {
    let mut set = HashSet::new();
    for char in s.chars() {
        set.insert(Segment::from_str(&char.to_string()).unwrap());
    }
    set
}

fn read_input() -> Vec<([DisplayState; 10], [DisplayState; 4])> {
    let file = File::open("inputs/8.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut inputs = Vec::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        let parts: Vec<&str> = line_.split(" | ").collect();
        let (first_part, second_part) = match parts[..] {
            [a, b] => (a, b),
            _ => panic!(),
        };
        let all_digits: [DisplayState; 10] = first_part
            .split(" ")
            .map(parse_set)
            .collect::<Vec<DisplayState>>()
            .try_into()
            .unwrap();
        let target_digits: [DisplayState; 4] = second_part
            .split(" ")
            .map(parse_set)
            .collect::<Vec<DisplayState>>()
            .try_into()
            .unwrap();
        inputs.push((all_digits, target_digits));
    }
    inputs
}

fn count_unique(input: Vec<([DisplayState; 10], [DisplayState; 4])>) -> u32 {
    let mut count = 0;
    for (_, target_digits) in input.iter() {
        for digit in target_digits {
            count += match digit.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }
    }
    count
}

fn main() {
    // let digits: [DisplayState; 10] = [
    //     parse_set("abcefg"),  // 0
    //     parse_set("cf"),      // 1
    //     parse_set("acdeg"),   // 2
    //     parse_set("acdfg"),   // 3
    //     parse_set("bcdf"),    // 4
    //     parse_set("abdfg"),   // 5
    //     parse_set("abdefg"),  // 6
    //     parse_set("acf"),     // 7
    //     parse_set("abcdefg"), // 8
    //     parse_set("abcdfg"),  // 9
    // ];
    let input = read_input();
    println!("{:?}", count_unique(input));
}
