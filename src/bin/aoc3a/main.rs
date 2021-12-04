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

fn bits_to_gamma_epsilon(bit_scores: Vec<i32>) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, score) in bit_scores.iter().enumerate() {
        if *score > 0 {
            gamma |= (1 << i);
        } else {
            epsilon |= (1 << i);
        }
    }

    (gamma, epsilon)
}

fn main() {
    let input = read_input();
    let scores = count_bits(input);
    let (gamma, epsilon) = bits_to_gamma_epsilon(scores);
    // let (pos, depth) = update_position(input);
    // let input_sum = window_sum(input);
    // let count = count_increases(input_sum);
    println!("{}", gamma*epsilon);
}
