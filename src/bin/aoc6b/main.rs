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
    let mut num_fish_with_days_left = [0u64; 9];
    let mut next_num_fish_with_days_left = [0u64; 9];
    for &fish in start_state.iter() {
        num_fish_with_days_left[fish as usize] += 1;
    }
    for day in 0..num_days {
        if day % 10 == 0 {
            println!("Day {}: {}", day, num_fish_with_days_left.iter().sum::<u64>());
        }
        next_num_fish_with_days_left[8] = num_fish_with_days_left[0];
        for i in 1..9usize {
            next_num_fish_with_days_left[i - 1] = num_fish_with_days_left[i];
        }
        next_num_fish_with_days_left[6] += num_fish_with_days_left[0];
        num_fish_with_days_left.clone_from(&next_num_fish_with_days_left);
    }
    num_fish_with_days_left.iter().sum()
}

fn main() {
    let input = read_input();
    let num = simulate_fish(input, 256);
    println!("{}", num);
}
