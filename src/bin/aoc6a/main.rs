use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<u8> {
    let file = File::open("inputs/6.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    br.lines().next().unwrap().unwrap().split(",").map(|n| n.parse().unwrap()).collect()
}

fn simulate_fish(start_state: Vec<u8>, num_days: u32) -> u64 {
    let mut state = start_state.clone();
    for _ in 0..num_days {
        for i in 0..state.len() {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }
    }
    state.len() as u64
}

fn main() {
    let input = read_input();
    let num = simulate_fish(input, 80);
    println!("{}", num);
}
