use phf::{phf_map, Map};
use std::fs::File;
use std::io::{BufRead, BufReader};

static DELIMITERS: Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

const DELIMITER_VALUES: Map<char, u64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
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

fn get_completion_score(line: &String) -> Option<u64> {
    let mut stack = vec![];
    for c in line.chars() {
        if DELIMITERS.contains_key(&c) {
            stack.push(c);
        } else if c == DELIMITERS[&stack.last().unwrap()] {
            stack.pop();
        } else {
            return None;
        }
    }
    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        score += DELIMITER_VALUES[&DELIMITERS[&c]];
    }
    Some(score)
}

fn main() {
    let lines = read_input();
    let mut scores = lines.iter().map(get_completion_score).filter_map(|x| x).collect::<Vec<u64>>();
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
