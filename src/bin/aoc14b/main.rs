use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (HashMap<(char, char), u64>, Vec<((char, char), ((char, char), (char, char)))>) {
    let file = File::open("inputs/14.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut lines = br.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut pair_counts = HashMap::new();
    for (c1, c2) in first_line.chars().zip(first_line.chars().skip(1)) {
        *pair_counts.entry((c1, c2)).or_insert(0) += 1;
    }
    lines.next();
    let mut rules = Vec::new();
    for line in lines {
        let line_ = line.unwrap();
        let (pattern, insert) = match line_.split(" -> ").collect::<Vec<_>>()[..] {
            [pattern, insert] => (pattern.to_string(), insert.chars().next().unwrap()),
            _ => panic!(),
        };
        let pattern_: (char, char) = match pattern.chars().collect::<Vec<_>>()[..] {
            [a, b] => (a, b),
            _ => panic!(),
        };
        rules.push((pattern_, ((pattern_.0, insert), (insert, pattern_.1))));
    }

    (pair_counts, rules)
}

fn run_step(pair_counts: &HashMap<(char, char), u64>, rules: &Vec<((char, char), ((char, char), (char, char)))>) -> HashMap<(char, char), u64> {
    let mut pair_counts_new = pair_counts.clone();
    for (pattern, insert) in rules.iter() {
        match pair_counts.get(pattern) {
            Some(count) => {
                *pair_counts_new.entry(*pattern).or_default() -= count;
                *pair_counts_new.entry(insert.0).or_insert(0) += count;
                *pair_counts_new.entry(insert.1).or_insert(0) += count;
            },
            None => {},
        }
    }
    pair_counts_new
}

fn calculate_score(pair_counts: &HashMap<(char, char), u64>) -> u64 {
    let mut char_counts = HashMap::new();
    for ((c1, c2), count) in pair_counts.iter() {
        *char_counts.entry(c1).or_insert(0) += count;
        *char_counts.entry(c2).or_insert(0) += count;
    }
    for (count) in char_counts.values_mut() {
        *count = (*count + 1) / 2;
    }
    // println!("{:?}", char_counts);
    let max_count = char_counts.values().max().unwrap();
    let min_count = char_counts.values().min().unwrap();
    max_count - min_count
}

fn main() {
    let (mut pair_counts, rules) = read_input();
    for step in 0..40 {
        pair_counts = run_step(&pair_counts, &rules);
        println!("{}: {}", step + 1, calculate_score(&pair_counts));
    }
}
