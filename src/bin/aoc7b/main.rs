use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<i32> {
    let file = File::open("inputs/7.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    br.lines().next().unwrap().unwrap().split(",").map(|n| n.parse().unwrap()).collect()
}

fn find_least_fuel_to_align(start_positions: Vec<i32>) -> i64 {
    let &min = start_positions.iter().min().unwrap();
    let &max = start_positions.iter().max().unwrap();
    let mut best_position = -1;
    let mut best_fuel_used = i64::MAX;
    for pos in min..max {
        let mut fuel_used = 0;
        for &crab_pos in start_positions.iter() {
            let dist = (crab_pos - pos).abs();
            let cost = dist * (dist + 1) / 2;
            fuel_used += cost as i64;
        }
        if fuel_used < best_fuel_used {
            best_position = pos;
            best_fuel_used = fuel_used;
        }
    }
    best_fuel_used
}

fn main() {
    let input = read_input();
    let num = find_least_fuel_to_align(input);
    println!("{}", num);
}
