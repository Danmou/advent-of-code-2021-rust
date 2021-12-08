use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;
use strum_macros::EnumString;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumString, EnumIter)]
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

fn parse_segments(input: &Vec<([DisplayState; 10], [DisplayState; 4])>, true_digits: &[DisplayState; 10]) -> Vec<u64> {
    let mut numbers = vec![];
    for (input_digits, target_digits) in input.iter() {
        let mapping = get_segment_mapping(input_digits, true_digits);
        let mut number_str: String = String::new();
        for digit in target_digits {
            let true_digit = map_digit(digit, &mapping);
            let (num, _) = true_digits.iter().enumerate().find(|(_, x)| **x == true_digit).unwrap();
            number_str += &*num.to_string();
        }
        let number = number_str.parse().unwrap();
        numbers.push(number);
    }
    numbers
}

fn get_segment_mapping(input_digits: &[DisplayState; 10], true_digits: &[DisplayState; 10]) -> HashMap<Segment, Segment> {
    let input_stats = get_segment_statistics(input_digits);
    let true_stats = get_segment_statistics(true_digits);
    let mut mapping = HashMap::new();
    for (&seg1, stats1) in input_stats.iter() {
        for (&seg2, stats2) in true_stats.iter() {
            if stats1 == stats2 {
                if mapping.contains_key(&seg1) {
                    panic!();
                }
                mapping.insert(seg1, seg2);
            }
        }
        if !mapping.contains_key(&seg1) {
            panic!();
        }
    }
    mapping
}

fn get_segment_statistics(digits: &[DisplayState; 10]) -> HashMap<Segment, HashMap<usize, usize>> {
    let mut stats = HashMap::new();
    for seg in Segment::iter() {
        let mut counts = HashMap::new();
        for seg2 in Segment::iter() {
            if seg2 == seg { continue; }
            let count: usize = digits.iter().filter(|&x| x.contains(&seg) && x.contains(&seg2)).count();
            *counts.entry(count).or_insert(0) += 1;
        }
        stats.insert(seg, counts);
    }
    stats
}

fn map_digit(digit: &DisplayState, mapping: &HashMap<Segment, Segment>) -> DisplayState {
    let mut new = HashSet::new();
    for segment in digit.iter() {
        new.insert(*mapping.get(segment).unwrap());
    }
    new
}

fn main() {
    let true_digits: [DisplayState; 10] = [
        parse_set("abcefg"),  // 0
        parse_set("cf"),      // 1
        parse_set("acdeg"),   // 2
        parse_set("acdfg"),   // 3
        parse_set("bcdf"),    // 4
        parse_set("abdfg"),   // 5
        parse_set("abdefg"),  // 6
        parse_set("acf"),     // 7
        parse_set("abcdefg"), // 8
        parse_set("abcdfg"),  // 9
    ];
    let input = read_input();
    println!("{:?}", parse_segments(&input, &true_digits).iter().sum::<u64>());
}
