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

fn read_input() -> Vec<u16> {
    let file = File::open("inputs/3.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    // println!("{:?}", vec![br.lines().collect::<Vec<String>>()[0].unwrap()]);
    br.lines().map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap()).collect()
}

fn count_bits(input: Vec<u16>) -> Vec<i32> {
    let mut scores = vec![0; 12];
    for num in input.iter() {
        for (i, score) in scores.iter_mut().enumerate() {
            if (num & (1 << i)) > 0 {
                *score += 1;
            } else {
                *score -= 1;
            }
        }
    }
    return scores
}

fn oxygen_generator_rating(input: Vec<u16>) -> u32 {
    let mut inp = input.clone();
    for bit in (0..12).rev() {
        let score = count_bits(inp.clone())[bit];
        inp = inp.iter().filter(|&&x| {
            if score >= 0 {
                (x & (1 << bit)) > 0
            } else {
                (x & (1 << bit)) == 0
            }
        }).cloned().collect();
        if inp.len() == 1 {
            return inp[0] as u32;
        }
    }
    panic!();
}

fn co2_scrubber_rating(input: Vec<u16>) -> u32 {
    let mut inp = input.clone();
    for bit in (0..12).rev() {
        let score = count_bits(inp.clone())[bit];
        inp = inp.iter().filter(|&&x| {
            if score >= 0 {
                (x & (1 << bit)) == 0
            } else {
                (x & (1 << bit)) > 0
            }
        }).cloned().collect();
        if inp.len() == 1 {
            return inp[0] as u32;
        }
    }
    panic!();
}

fn main() {
    let input = read_input();
    let rat1 = oxygen_generator_rating(input.clone());
    let rat2 = co2_scrubber_rating(input.clone());
    println!("{}", rat1*rat2);
}
