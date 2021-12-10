use phf::{phf_map, Map};
use std::fs::File;
use std::io::{BufRead, BufReader};

static DELIMITERS: Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

const DELIMITER_VALUES: Map<char, u32> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};

fn read_input() -> Vec<String> {
    let file = File::open("inputs/10.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    br.lines().map(|line| line.unwrap()).collect()
}

fn get_corruption_value(line: &String) -> u32 {
    let mut stack = vec![];
    for c in line.chars() {
        if DELIMITERS.contains_key(&c) {
            stack.push(c);
        } else if c == DELIMITERS[&stack.last().unwrap()] {
            stack.pop();
        } else {
            return DELIMITER_VALUES[&c];
        }
    }
    0
}

fn main() {
    let lines = read_input();
    println!("{}", lines.iter().map(get_corruption_value).sum::<u32>());
}
