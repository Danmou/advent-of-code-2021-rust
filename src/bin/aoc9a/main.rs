use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<Vec<u8>> {
    let file = File::open("inputs/9.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut map = Vec::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        map.push(
            line_
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>(),
        );
    }
    map
}

fn sum_lowest_points(map: Vec<Vec<u8>>) -> u32 {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let val = map[i][j];
            if i > 0 && map[i - 1][j] <= val {
                continue;
            }
            if i < map.len() - 1 && map[i + 1][j] <= val {
                continue;
            }
            if j > 0 && map[i][j - 1] <= val {
                continue;
            }
            if j < map[i].len() - 1 && map[i][j + 1] <= val {
                continue;
            }
            sum += val as u32 + 1;
        }
    }
    sum
}

fn main() {
    let map = read_input();
    println!("{}", sum_lowest_points(map));
}
