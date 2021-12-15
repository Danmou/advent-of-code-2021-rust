use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> (String, Vec<(String, char)>) {
    let file = File::open("inputs/14.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut lines = br.lines();
    let template = lines.next().unwrap().unwrap();
    lines.next();
    let mut rules = Vec::new();
    for line in lines {
        let line_ = line.unwrap();
        let (pattern, insert) = match line_.split(" -> ").collect::<Vec<_>>()[..] {
            [pattern, insert] => (pattern.to_string(), insert.chars().next().unwrap()),
            _ => panic!(),
        };
        rules.push((pattern, insert));
    }

    (template, rules)
}

fn run_step(polymer: &mut String, rules: &Vec<(String, char)>) {
    let mut insertions = Vec::new();
    for (pattern, insert) in rules.iter() {
        for i in 0..polymer.len() - 1 {
            if polymer[i..].starts_with(pattern) {
                insertions.push((i + 1, *insert));
            }
        }
    }

    insertions.sort_unstable();

    for (i, (idx, ins)) in insertions.iter().enumerate() {
        polymer.insert(i + idx, *ins);
    }
}

fn calculate_score(polymer: &String) -> u32 {
    let mut char_counts = HashMap::new();
    for c in polymer.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    // println!("{:?}", char_counts);
    let max_count = char_counts.values().max().unwrap();
    let min_count = char_counts.values().min().unwrap();
    max_count - min_count
}

fn main() {
    let (mut polymer, rules) = read_input();
    println!("{}", polymer);
    for step in 0..10 {
        run_step(&mut polymer, &rules);
        // println!("{}: {}", step, polymer);
    }
    println!("{}", calculate_score(&polymer));
}
